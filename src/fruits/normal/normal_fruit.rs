//! 普通果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};

/// 普通果实
///
/// 红色果实，吃到后蛇身增长1格，+1分
pub struct NormalFruit {
    config: FruitConfig,
}

impl NormalFruit {
    pub fn new() -> Self {
        NormalFruit {
            config: FruitConfig {
                id: "normal",
                name: "普通果实",
                category: FruitCategory::Normal,
                color: RED,
                lifetime: 0.0, // 永久存在
                spawn_weight: 100,
                unlock_length: 0,
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for NormalFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for NormalFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // Combo加分
        let combo_bonus = ctx.combo_state.count.min(5);
        ConsumeResult::AddScore(1 + combo_bonus)
    }
}
