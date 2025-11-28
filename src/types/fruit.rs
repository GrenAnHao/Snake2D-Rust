//! 果实实例类型定义
//!
//! 运行时果实数据，与 FruitRegistry 配合使用

use macroquad::prelude::*;

/// 果实实例（运行时数据）
///
/// 表示游戏中的一个果实实例，包含位置、类型和生命周期信息。
#[derive(Clone)]
pub struct Fruit {
    /// 网格位置
    pub pos: IVec2,
    /// 果实类型ID，对应 FruitRegistry 中的 ID
    pub type_id: &'static str,
    /// 生成时间戳
    pub spawn_time: f32,
    /// 存在时长（0=永久）
    pub lifetime: f32,
}

impl Fruit {
    /// 创建新的果实实例
    pub fn new(pos: IVec2, type_id: &'static str, spawn_time: f32, lifetime: f32) -> Self {
        Fruit {
            pos,
            type_id,
            spawn_time,
            lifetime,
        }
    }

    /// 检查果实是否已过期
    pub fn is_expired(&self, current_time: f32) -> bool {
        self.lifetime > 0.0 && current_time - self.spawn_time >= self.lifetime
    }

    /// 计算剩余时间比例（用于进度条）
    pub fn remaining_ratio(&self, current_time: f32) -> f32 {
        if self.lifetime <= 0.0 {
            1.0
        } else {
            (1.0 - (current_time - self.spawn_time) / self.lifetime).max(0.0)
        }
    }
}
