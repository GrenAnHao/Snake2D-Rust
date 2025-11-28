//! 减速果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, SLOW_DURATION};

/// 减速果实
///
/// 橙色果实，吃到后移动速度变慢5秒，蛇身抽搐
pub struct SlowFruit {
    config: FruitConfig,
}

impl SlowFruit {
    pub fn new() -> Self {
        SlowFruit {
            config: FruitConfig {
                id: "slow",
                name: "减速果实",
                category: FruitCategory::Trap,
                color: Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 20,
                unlock_length: 0,
                immune_to_buffs: true,
            },
        }
    }
}

impl Default for SlowFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SlowFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活减速
        ctx.buff_state.slow_active = true;
        ctx.buff_state.slow_timer = SLOW_DURATION;

        ConsumeResult::ResetCombo
    }
}
