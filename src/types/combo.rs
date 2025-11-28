//! Combo和残影相关类型定义

use macroquad::prelude::*;

/// Combo时间窗口（秒）
pub const COMBO_WINDOW: f32 = 1.5;
/// 残影存在时间（秒）
pub const AFTERIMAGE_LIFETIME: f32 = 0.3;

/// Combo系统结构体
///
/// 追踪连续吃果实的连击数
#[derive(Clone)]
pub struct ComboState {
    /// 当前连击数
    pub count: u32,
    /// 上次吃果实的游戏时间
    pub last_eat_time: f32,
    /// combo显示剩余时间
    pub display_timer: f32,
}

impl Default for ComboState {
    fn default() -> Self {
        ComboState {
            count: 0,
            last_eat_time: -10.0,
            display_timer: 0.0,
        }
    }
}

/// 残影结构体
///
/// 速度模式下蛇移动时留下的蓝色残影效果
#[derive(Clone)]
pub struct Afterimage {
    /// 残影的蛇身位置快照
    pub positions: Vec<IVec2>,
    /// 透明度
    pub alpha: f32,
    /// 生成时间
    pub spawn_time: f32,
}
