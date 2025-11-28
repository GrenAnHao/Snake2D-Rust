//! # 蛇状态和移动逻辑
//!
//! 封装蛇的状态管理和移动行为，是游戏的核心数据结构。
//!
//! ## 设计说明
//!
//! 蛇使用 `Vec<IVec2>` 存储身体位置，索引 0 为头部。
//! 移动时采用"插入头部、移除尾部"的方式，保持长度不变。
//!
//! ## 移动算法
//!
//! ```text
//! 移动前: [H][1][2][T]  (H=头, T=尾)
//!              ↓
//! 1. 计算新头部位置
//! 2. 插入新头部: [N][H][1][2][T]
//! 3. 移除尾部:   [N][H][1][2]
//!              ↓
//! 移动后: [H'][1'][2'][T']
//! ```
//!
//! ## 插值渲染
//!
//! `prev_body` 存储上一帧的位置，用于在帧间插值实现平滑移动。
//! 渲染时根据累积时间计算插值比例。

use macroquad::prelude::*;
use crate::constants::{GRID_W, GRID_H};

/// 移动结果枚举
///
/// 表示蛇移动一步后的结果。游戏主循环根据此结果决定后续行为。
///
/// ## 处理逻辑
///
/// - `Normal`: 继续游戏，检查食物和果实碰撞
/// - `WallCollision`: 非包围模式下撞墙，游戏结束（除非有免疫）
/// - `SelfCollision`: 撞到自己，游戏结束（除非有免疫）
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveResult {
    /// 正常移动成功
    ///
    /// 包含新头部位置，用于后续碰撞检测
    Normal(IVec2),

    /// 撞墙
    ///
    /// 仅在非包围模式（wrap=false）下发生
    WallCollision,

    /// 撞到自己
    ///
    /// 新头部位置与身体某节重叠
    SelfCollision,
}

/// 蛇结构体
///
/// 管理蛇的状态和移动逻辑。这是游戏的核心数据结构。
///
/// ## 字段说明
///
/// - `body`: 蛇身位置列表，`body[0]` 是头部
/// - `dir`: 当前移动方向，每帧移动一格
/// - `prev_body`: 上一帧位置，用于插值渲染
///
/// ## 不变量
///
/// - `body.len() >= 1`（至少有头部）
/// - `dir` 是四方向之一：(1,0), (-1,0), (0,1), (0,-1)
#[derive(Clone)]
pub struct Snake {
    /// 蛇身位置列表
    ///
    /// 索引 0 为头部，最后一个元素为尾部。
    /// 相邻元素在网格上相邻（曼哈顿距离为 1）。
    pub body: Vec<IVec2>,

    /// 当前移动方向
    ///
    /// 每次移动时，头部位置 += dir。
    /// 只能是四方向之一，不能是 (0,0) 或对角线。
    pub dir: IVec2,

    /// 上一帧的蛇身位置
    ///
    /// 用于插值渲染，实现平滑移动效果。
    /// 在 `move_forward` 开始时更新。
    pub prev_body: Vec<IVec2>,
}

impl Snake {
    /// 创建新的蛇，初始位置在屏幕中央
    pub fn new() -> Self {
        let body = vec![
            ivec2(GRID_W / 2, GRID_H / 2),     // 头部
            ivec2(GRID_W / 2 - 1, GRID_H / 2), // 身体
            ivec2(GRID_W / 2 - 2, GRID_H / 2), // 尾部
        ];
        Snake {
            prev_body: body.clone(),
            body,
            dir: ivec2(1, 0), // 初始向右
        }
    }

    /// 获取蛇头位置
    pub fn head(&self) -> IVec2 {
        self.body[0]
    }

    /// 获取蛇身长度
    pub fn len(&self) -> usize {
        self.body.len()
    }

    /// 检查蛇是否为空
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    /// 设置移动方向（防止反向）
    pub fn set_direction(&mut self, new_dir: IVec2) {
        // 防止反向掉头
        if new_dir.x != -self.dir.x || new_dir.y != -self.dir.y {
            self.dir = new_dir;
        }
    }

    /// 向前移动一步
    ///
    /// # 参数
    /// - `wrap`: 是否启用包围模式（穿墙）
    /// - `can_pass_self`: 是否可以穿过自己（护盾/幽灵/沙虫模式）
    ///
    /// # 返回
    /// 移动结果，包含新头部位置或碰撞类型
    pub fn move_forward(&mut self, wrap: bool, can_pass_self: bool) -> MoveResult {
        // 保存上一帧位置
        self.prev_body = self.body.clone();

        let head = self.head();
        let mut nx = head.x + self.dir.x;
        let mut ny = head.y + self.dir.y;

        // 处理边界（可穿过自己的模式通常也可以穿墙）
        if can_pass_self || wrap {
            if nx < 0 { nx = GRID_W - 1; }
            if nx >= GRID_W { nx = 0; }
            if ny < 0 { ny = GRID_H - 1; }
            if ny >= GRID_H { ny = 0; }
        } else {
            // 非包围模式，检查撞墙
            if nx < 0 || nx >= GRID_W || ny < 0 || ny >= GRID_H {
                return MoveResult::WallCollision;
            }
        }

        let new_head = ivec2(nx, ny);

        // 检查自撞（护盾/幽灵/沙虫模式下可穿过自己）
        if !can_pass_self && self.body.iter().skip(1).any(|&p| p == new_head) {
            return MoveResult::SelfCollision;
        }

        // 移动：插入新头部，移除尾部
        self.body.insert(0, new_head);
        self.body.pop();

        MoveResult::Normal(new_head)
    }

    /// 增长一节（吃到食物时调用）
    pub fn grow(&mut self) {
        if let Some(tail) = self.body.last().cloned() {
            self.body.push(tail);
        }
    }

    /// 缩短指定节数
    pub fn shrink(&mut self, count: usize) {
        for _ in 0..count {
            if self.body.len() > 1 {
                self.body.pop();
            }
        }
    }

    /// 头尾反转
    pub fn reverse(&mut self) {
        self.body.reverse();
        // 更新方向为新头部的方向
        if self.body.len() > 1 {
            let new_head = self.body[0];
            let second = self.body[1];
            self.dir = ivec2(new_head.x - second.x, new_head.y - second.y);
        }
    }

    /// 重置到初始状态
    pub fn reset(&mut self) {
        self.body = vec![
            ivec2(GRID_W / 2, GRID_H / 2),
            ivec2(GRID_W / 2 - 1, GRID_H / 2),
            ivec2(GRID_W / 2 - 2, GRID_H / 2),
        ];
        self.prev_body = self.body.clone();
        self.dir = ivec2(1, 0);
    }

    /// 传送蛇（整条蛇加上偏移量）
    pub fn teleport(&mut self, offset: IVec2) {
        for seg in self.body.iter_mut() {
            seg.x += offset.x;
            seg.y += offset.y;
            // 处理边界包围
            if seg.x < 0 { seg.x += GRID_W; }
            if seg.x >= GRID_W { seg.x -= GRID_W; }
            if seg.y < 0 { seg.y += GRID_H; }
            if seg.y >= GRID_H { seg.y -= GRID_H; }
        }
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_new() {
        let snake = Snake::new();
        assert_eq!(snake.len(), 3);
        assert_eq!(snake.dir, ivec2(1, 0));
    }

    #[test]
    fn test_snake_move_preserves_length() {
        let mut snake = Snake::new();
        let initial_len = snake.len();
        snake.move_forward(true, false);
        assert_eq!(snake.len(), initial_len);
    }

    #[test]
    fn test_snake_grow() {
        let mut snake = Snake::new();
        let initial_len = snake.len();
        snake.grow();
        assert_eq!(snake.len(), initial_len + 1);
    }

    #[test]
    fn test_snake_shrink() {
        let mut snake = Snake::new();
        snake.grow();
        snake.grow();
        let len_before = snake.len();
        snake.shrink(2);
        assert_eq!(snake.len(), len_before - 2);
    }

    #[test]
    fn test_snake_reverse() {
        let mut snake = Snake::new();
        let original_head = snake.head();
        let original_tail = *snake.body.last().unwrap();
        snake.reverse();
        assert_eq!(snake.head(), original_tail);
        assert_eq!(*snake.body.last().unwrap(), original_head);
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: modular-migration, Property 1: Snake Movement Preserves Length**
    // *For any* snake body and valid direction, moving forward without eating
    // should preserve the snake length (body.len() remains constant).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_move_preserves_length(
            dir_x in prop_oneof![Just(-1i32), Just(0i32), Just(1i32)],
            dir_y in prop_oneof![Just(-1i32), Just(0i32), Just(1i32)],
            wrap in proptest::bool::ANY,
            ghost in proptest::bool::ANY,
        ) {
            // 确保方向有效（不能是(0,0)）
            prop_assume!(dir_x != 0 || dir_y != 0);
            // 确保是四方向之一
            prop_assume!((dir_x == 0) != (dir_y == 0));

            let mut snake = Snake::new();
            snake.dir = ivec2(dir_x, dir_y);
            let initial_len = snake.len();

            let result = snake.move_forward(wrap, ghost);

            // 只有正常移动时检查长度
            if let MoveResult::Normal(_) = result {
                prop_assert_eq!(snake.len(), initial_len);
            }
        }
    }
}
