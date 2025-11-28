//! 生成逻辑模块
//!
//! 提供食物、果实、传送门的生成函数

use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use crate::constants::{GRID_W, GRID_H};
use crate::types::{Fruit, Portal};
use crate::fruits::{FruitRegistry, FruitCategory};

/// 生成不与蛇身和现有果实重叠的随机位置
///
/// # 参数
/// - `snake`: 蛇身位置列表
/// - `fruits`: 现有果实列表
/// - `rng`: 随机数生成器
///
/// # 返回
/// - `Some(pos)`: 找到的空闲位置
/// - `None`: 无法找到空闲位置（尝试100次后）
pub fn spawn_position(
    snake: &[IVec2],
    fruits: &[Fruit],
    rng: &mut ThreadRng,
) -> Option<IVec2> {
    for _ in 0..100 {
        let p = ivec2(rng.gen_range(0..GRID_W), rng.gen_range(0..GRID_H));
        if !snake.iter().any(|&s| s == p) && !fruits.iter().any(|f| f.pos == p) {
            return Some(p);
        }
    }
    None
}

/// 生成食物位置（只检查蛇身）
pub fn spawn_food(snake: &[IVec2], rng: &mut ThreadRng) -> IVec2 {
    loop {
        let p = ivec2(rng.gen_range(0..GRID_W), rng.gen_range(0..GRID_H));
        if !snake.iter().any(|&s| s == p) {
            return p;
        }
    }
}

/// 使用 FruitRegistry 生成果实
///
/// # 参数
/// - `registry`: 果实注册表
/// - `category`: 果实类别
/// - `snake`: 蛇身位置列表
/// - `fruits`: 现有果实列表
/// - `game_time`: 当前游戏时间
/// - `rng`: 随机数生成器
///
/// # 返回
/// 生成的果实实例，如果无法生成则返回 None
pub fn spawn_fruit(
    registry: &FruitRegistry,
    category: FruitCategory,
    snake: &[IVec2],
    fruits: &[Fruit],
    game_time: f32,
    rng: &mut ThreadRng,
) -> Option<Fruit> {
    let pos = spawn_position(snake, fruits, rng)?;
    let type_id = registry.random_by_category(category, snake.len(), rng)?;
    let config = registry.get_config(type_id)?;

    Some(Fruit::new(pos, type_id, game_time, config.lifetime))
}

/// 生成传送门（成对出现）
pub fn spawn_portal(
    snake: &[IVec2],
    fruits: &[Fruit],
    game_time: f32,
    rng: &mut ThreadRng,
) -> Option<Portal> {
    use crate::types::PORTAL_LIFETIME;

    // 生成第一个位置
    let pos_a = spawn_position(snake, fruits, rng)?;

    // 临时创建一个假果实来避免第二个位置与第一个重叠
    let mut temp_fruits: Vec<Fruit> = fruits.to_vec();
    temp_fruits.push(Fruit::new(pos_a, "temp", 0.0, 0.0));

    let pos_b = spawn_position(snake, &temp_fruits, rng)?;

    // 随机颜色
    let colors = [
        Color { r: 0.5, g: 0.0, b: 1.0, a: 1.0 }, // 紫色
        Color { r: 0.0, g: 1.0, b: 0.5, a: 1.0 }, // 青绿
        Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }, // 橙色
    ];
    let color = colors[rng.gen_range(0..colors.len())];

    Some(Portal {
        pos_a,
        pos_b,
        color,
        spawn_time: game_time,
        lifetime: PORTAL_LIFETIME,
    })
}

/// 更新果实列表，移除过期果实
///
/// # 返回
/// 过期果实的位置列表
pub fn update_fruits(fruits: &mut Vec<Fruit>, game_time: f32) -> Vec<IVec2> {
    let mut expired = vec![];
    fruits.retain(|f| {
        if f.is_expired(game_time) {
            expired.push(f.pos);
            false
        } else {
            true
        }
    });
    expired
}

/// 更新传送门列表，移除过期传送门
pub fn update_portals(portals: &mut Vec<Portal>, game_time: f32) {
    portals.retain(|p| game_time - p.spawn_time < p.lifetime);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::rand::thread_rng;

    #[test]
    fn test_spawn_food_not_on_snake() {
        let snake = vec![ivec2(5, 5), ivec2(4, 5), ivec2(3, 5)];
        let mut rng = thread_rng();
        for _ in 0..100 {
            let food = spawn_food(&snake, &mut rng);
            assert!(!snake.contains(&food));
        }
    }

    #[test]
    fn test_spawn_position_not_on_snake_or_fruits() {
        let snake = vec![ivec2(5, 5), ivec2(4, 5)];
        let fruits = vec![Fruit::new(ivec2(10, 10), "normal", 0.0, 0.0)];
        let mut rng = thread_rng();

        for _ in 0..100 {
            if let Some(pos) = spawn_position(&snake, &fruits, &mut rng) {
                assert!(!snake.contains(&pos));
                assert!(!fruits.iter().any(|f| f.pos == pos));
            }
        }
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use ::rand::thread_rng;

    // **Feature: modular-migration, Property 3: Spawn Position Validity**
    // *For any* snake body and existing fruits, a spawned position should never
    // overlap with the snake body or existing fruit positions.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_spawn_position_no_overlap(
            snake_len in 3usize..20,
            fruit_count in 0usize..10,
        ) {
            let mut rng = thread_rng();

            // 生成蛇身（直线）
            let snake: Vec<IVec2> = (0..snake_len as i32)
                .map(|i| ivec2(i % GRID_W, (i / GRID_W) % GRID_H))
                .collect();

            // 生成一些果实
            let fruits: Vec<Fruit> = (0..fruit_count)
                .map(|i| {
                    let x = (snake_len as i32 + i as i32) % GRID_W;
                    let y = ((snake_len as i32 + i as i32) / GRID_W) % GRID_H;
                    Fruit::new(ivec2(x, y), "test", 0.0, 0.0)
                })
                .collect();

            // 尝试生成位置
            if let Some(pos) = spawn_position(&snake, &fruits, &mut rng) {
                // 验证不与蛇身重叠
                prop_assert!(!snake.contains(&pos), "Position overlaps with snake");
                // 验证不与果实重叠
                prop_assert!(!fruits.iter().any(|f| f.pos == pos), "Position overlaps with fruit");
            }
        }
    }
}
