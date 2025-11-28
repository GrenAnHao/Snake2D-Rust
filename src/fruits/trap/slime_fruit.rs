//! 粘液果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, SLIME_DURATION};

/// 粘液果实
///
/// 绿色果实，吃到后蛇身粘稠5秒，有拖拽抖动
pub struct SlimeFruit {
    config: FruitConfig,
}

impl SlimeFruit {
    pub fn new() -> Self {
        SlimeFruit {
            config: FruitConfig {
                id: "slime",
                name: "粘液果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.3, g: 0.8, b: 0.3, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 15,
                unlock_length: 0,
                immune_to_buffs: true,
            },
        }
    }
}

impl Default for SlimeFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SlimeFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活粘液
        ctx.buff_state.slime_active = true;
        ctx.buff_state.slime_timer = SLIME_DURATION;

        ConsumeResult::ResetCombo
    }
}
