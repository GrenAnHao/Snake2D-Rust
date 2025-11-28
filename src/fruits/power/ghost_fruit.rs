//! 幽灵果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SHIELD_FRUIT_LIFETIME, GHOST_DURATION};

/// 幽灵果实
///
/// 白色半透明果实，5秒可穿墙穿身
pub struct GhostFruit {
    config: FruitConfig,
}

impl GhostFruit {
    pub fn new() -> Self {
        GhostFruit {
            config: FruitConfig {
                id: "ghost",
                name: "幽灵果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 },
                lifetime: SHIELD_FRUIT_LIFETIME,
                spawn_weight: 20,
                unlock_length: 10,
                immune_to_buffs: false,
            },
        }
    }
}

impl Default for GhostFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for GhostFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        ctx.buff_state.ghost_active = true;
        ctx.buff_state.ghost_timer = GHOST_DURATION;
        ConsumeResult::Continue
    }
}
