//! 速度果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SHIELD_FRUIT_LIFETIME, SPEED_DURATION};

/// 速度果实
///
/// 蓝色果实，5秒移动速度加倍
pub struct SpeedFruit {
    config: FruitConfig,
}

impl SpeedFruit {
    pub fn new() -> Self {
        SpeedFruit {
            config: FruitConfig {
                id: "speed",
                name: "速度果实",
                category: FruitCategory::Power,
                color: Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 },
                lifetime: SHIELD_FRUIT_LIFETIME,
                spawn_weight: 25,
                unlock_length: 10,
                immune_to_buffs: false,
            },
        }
    }
}

impl Default for SpeedFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SpeedFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        ctx.buff_state.speed_active = true;
        ctx.buff_state.speed_timer = SPEED_DURATION;
        ConsumeResult::Continue
    }
}
