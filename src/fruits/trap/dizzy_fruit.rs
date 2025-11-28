//! 眩晕果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, DIZZY_DURATION};

/// 眩晕果实
///
/// 黄绿色果实，吃到后方向随机偏移5秒，醉酒效果
pub struct DizzyFruit {
    config: FruitConfig,
}

impl DizzyFruit {
    pub fn new() -> Self {
        DizzyFruit {
            config: FruitConfig {
                id: "dizzy",
                name: "眩晕果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.8, g: 1.0, b: 0.3, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 15,
                unlock_length: 0,
                immune_to_buffs: true,
            },
        }
    }
}

impl Default for DizzyFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for DizzyFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活眩晕
        ctx.buff_state.dizzy_active = true;
        ctx.buff_state.dizzy_timer = DIZZY_DURATION;

        ConsumeResult::ResetCombo
    }
}
