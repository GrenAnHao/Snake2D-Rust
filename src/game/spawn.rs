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

/// 过期果实信息
pub struct ExpiredFruit {
    pub pos: IVec2,
    pub type_id: &'static str,
}

/// 更新果实列表，移除过期果实
///
/// # 返回
/// 过期果实的信息列表（位置和类型）
pub fn update_fruits(fruits: &mut Vec<Fruit>, game_time: f32) -> Vec<ExpiredFruit> {
    let mut expired = vec![];
    fruits.retain(|f| {
        if f.is_expired(game_time) {
            expired.push(ExpiredFruit {
                pos: f.pos,
                type_id: f.type_id,
            });
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

use crate::fruits::FruitContext;
use crate::types::{BuffState, DamageState, Particle, ComboState};
use crate::game::AIManager;

/// 更新果实列表，处理过期果实并调用 on_expire 回调
///
/// 这是声明式架构的核心函数：每种果实的过期逻辑都在其自己的 on_expire 回调中定义，
/// 主循环不需要知道具体有哪些果实类型。
///
/// # 参数
/// - `fruits`: 果实列表
/// - `registry`: 果实注册表
/// - `snake`: 蛇身位置列表
/// - `dir`: 蛇移动方向
/// - `buff_state`: Buff状态
/// - `damage_state`: 受伤状态
/// - `particles`: 粒子列表
/// - `score`: 当前分数
/// - `combo_state`: Combo状态
/// - `ai_manager`: AI蛇管理器
/// - `food`: 食物位置
/// - `game_time`: 当前游戏时间
/// - `rng`: 随机数生成器
///
/// # 返回
/// 过期果实的信息列表（位置和类型）
#[allow(clippy::too_many_arguments)]
pub fn update_fruits_with_callbacks(
    fruits: &mut Vec<Fruit>,
    registry: &FruitRegistry,
    snake: &mut Vec<IVec2>,
    dir: &mut IVec2,
    buff_state: &mut BuffState,
    damage_state: &mut DamageState,
    particles: &mut Vec<Particle>,
    score: &mut u32,
    combo_state: &mut ComboState,
    ai_manager: &mut AIManager,
    food: &mut IVec2,
    game_time: f32,
    rng: &mut ThreadRng,
) -> Vec<ExpiredFruit> {
    let mut expired = vec![];
    
    // 收集过期果实的索引和信息
    let expired_indices: Vec<(usize, IVec2, &'static str)> = fruits
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_expired(game_time))
        .map(|(i, f)| (i, f.pos, f.type_id))
        .collect();
    
    // 为每个过期果实调用 on_expire 回调
    for &(_, pos, type_id) in &expired_indices {
        if let Some(behavior) = registry.get(type_id) {
            // 创建上下文 - 注意：这里需要临时借用 fruits
            // 由于我们还没有移除过期果实，需要小心处理
            let mut ctx = FruitContext {
                snake,
                dir,
                buff_state,
                particles,
                damage_state,
                score,
                combo_state,
                rng,
                game_time,
                fruit_pos: pos,
                ai_manager,
                food,
                fruits,
            };
            
            // 调用 on_expire 回调 - 所有特定逻辑都在各果实的实现中
            behavior.on_expire(&mut ctx);
        }
        
        expired.push(ExpiredFruit { pos, type_id });
    }
    
    // 移除过期果实（从后往前移除以保持索引正确）
    for &(idx, _, _) in expired_indices.iter().rev() {
        fruits.remove(idx);
    }
    
    expired
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
