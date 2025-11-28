//! 护盾果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SHIELD_FRUIT_LIFETIME, SHIELD_DURATION};

/// 护盾果实
///
/// 金色果实，10秒无敌护盾
pub struct ShieldFruit {
    config: FruitConfig,
}

impl ShieldFruit {
    pub fn new() -> Self {
        ShieldFruit {
            config: FruitConfig {
                id: "shield",
                name: "护盾果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 },
                lifetime: SHIELD_FRUIT_LIFETIME,
                spawn_weight: 30,
                unlock_length: 10,
                immune_to_buffs: false,
            },
        }
    }
}

impl Default for ShieldFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for ShieldFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        ctx.buff_state.shield_active = true;
        ctx.buff_state.shield_timer = SHIELD_DURATION;
        ConsumeResult::Continue
    }
}
