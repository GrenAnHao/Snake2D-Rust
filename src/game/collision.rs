//! 碰撞检测模块
//!
//! 提供各种碰撞检测函数

use macroquad::prelude::*;
use crate::constants::{GRID_W, GRID_H};
use crate::types::{Fruit, Portal};

/// 检查是否撞墙
///
/// # 参数
/// - `pos`: 要检查的位置
/// - `wrap`: 是否启用包围模式
///
/// # 返回
/// 如果撞墙返回 true
pub fn check_wall_collision(pos: IVec2, wrap: bool) -> bool {
    if wrap {
        false // 包围模式下不会撞墙
    } else {
        pos.x < 0 || pos.x >= GRID_W || pos.y < 0 || pos.y >= GRID_H
    }
}

/// 检查是否撞到自己
///
/// # 参数
/// - `pos`: 要检查的位置（通常是新头部位置）
/// - `body`: 蛇身位置列表（不包括即将移除的尾部）
///
/// # 返回
/// 如果撞到自己返回 true
pub fn check_self_collision(pos: IVec2, body: &[IVec2]) -> bool {
    // 跳过第一个（头部）和最后一个（即将移除的尾部）
    body.iter().skip(1).take(body.len().saturating_sub(2)).any(|&p| p == pos)
}

/// 检查是否碰到果实
///
/// # 参数
/// - `pos`: 要检查的位置
/// - `fruits`: 果实列表
///
/// # 返回
/// 如果碰到果实，返回果实在列表中的索引
pub fn check_fruit_collision(pos: IVec2, fruits: &[Fruit]) -> Option<usize> {
    fruits.iter().position(|f| f.pos == pos)
}

/// 检查是否碰到传送门
///
/// # 参数
/// - `pos`: 要检查的位置
/// - `portals`: 传送门列表
///
/// # 返回
/// 如果碰到传送门，返回 (进入位置, 出口位置)
pub fn check_portal_collision(pos: IVec2, portals: &[Portal]) -> Option<(IVec2, IVec2)> {
    for portal in portals {
        if pos == portal.pos_a {
            return Some((portal.pos_a, portal.pos_b));
        } else if pos == portal.pos_b {
            return Some((portal.pos_b, portal.pos_a));
        }
    }
    None
}

/// 检查位置是否与蛇身重叠
pub fn overlaps_snake(pos: IVec2, snake_body: &[IVec2]) -> bool {
    snake_body.iter().any(|&s| s == pos)
}

/// 检查位置是否与果实重叠
pub fn overlaps_fruits(pos: IVec2, fruits: &[Fruit]) -> bool {
    fruits.iter().any(|f| f.pos == pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_collision_wrap() {
        assert!(!check_wall_collision(ivec2(-1, 0), true));
        assert!(!check_wall_collision(ivec2(GRID_W, 0), true));
    }

    #[test]
    fn test_wall_collision_no_wrap() {
        assert!(check_wall_collision(ivec2(-1, 0), false));
        assert!(check_wall_collision(ivec2(GRID_W, 0), false));
        assert!(!check_wall_collision(ivec2(0, 0), false));
    }

    #[test]
    fn test_self_collision() {
        let body = vec![ivec2(5, 5), ivec2(4, 5), ivec2(3, 5), ivec2(2, 5)];
        // 撞到身体中间
        assert!(check_self_collision(ivec2(4, 5), &body));
        // 不会撞到头部
        assert!(!check_self_collision(ivec2(5, 5), &body));
        // 不会撞到尾部（即将移除）
        assert!(!check_self_collision(ivec2(2, 5), &body));
    }

    #[test]
    fn test_fruit_collision() {
        let fruits = vec![
            Fruit::new(ivec2(10, 10), "normal", 0.0, 0.0),
            Fruit::new(ivec2(15, 15), "trap", 0.0, 5.0),
        ];
        assert_eq!(check_fruit_collision(ivec2(10, 10), &fruits), Some(0));
        assert_eq!(check_fruit_collision(ivec2(15, 15), &fruits), Some(1));
        assert_eq!(check_fruit_collision(ivec2(20, 20), &fruits), None);
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: modular-migration, Property 2: Collision Detection Correctness**
    // *For any* position and snake body, self-collision should be detected
    // if and only if the position exists in the body (excluding head and tail).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_self_collision_correctness(
            pos_x in 0i32..GRID_W,
            pos_y in 0i32..GRID_H,
            body_len in 3usize..10,
        ) {
            // 生成一条直线蛇身
            let body: Vec<IVec2> = (0..body_len as i32)
                .map(|i| ivec2(i % GRID_W, i / GRID_W))
                .collect();

            let pos = ivec2(pos_x, pos_y);
            let collision = check_self_collision(pos, &body);

            // 检查位置是否在身体中间（排除头和尾）
            let in_middle = body.iter()
                .skip(1)
                .take(body.len().saturating_sub(2))
                .any(|&p| p == pos);

            prop_assert_eq!(collision, in_middle);
        }

        #[test]
        fn prop_wall_collision_wrap_never_collides(
            pos_x in -10i32..GRID_W + 10,
            pos_y in -10i32..GRID_H + 10,
        ) {
            // 包围模式下永远不会撞墙
            prop_assert!(!check_wall_collision(ivec2(pos_x, pos_y), true));
        }

        #[test]
        fn prop_wall_collision_no_wrap(
            pos_x in -10i32..GRID_W + 10,
            pos_y in -10i32..GRID_H + 10,
        ) {
            let pos = ivec2(pos_x, pos_y);
            let collision = check_wall_collision(pos, false);
            let out_of_bounds = pos_x < 0 || pos_x >= GRID_W || pos_y < 0 || pos_y >= GRID_H;
            prop_assert_eq!(collision, out_of_bounds);
        }
    }
}
