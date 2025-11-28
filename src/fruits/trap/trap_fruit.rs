//! 陷阱果实实现（紫色，减蛇身）

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::TRAP_LIFETIME;
use crate::types::DamagePhase;

/// 陷阱果实
///
/// 紫色果实，吃到后蛇身减少2格，触发受伤动画
pub struct TrapFruit {
    config: FruitConfig,
}

impl TrapFruit {
    pub fn new() -> Self {
        TrapFruit {
            config: FruitConfig {
                id: "trap",
                name: "陷阱果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.6, g: 0.2, b: 0.8, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 35,
                unlock_length: 0,
                immune_to_buffs: true,
            },
        }
    }
}

impl Default for TrapFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for TrapFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 蛇太短直接死亡
        if ctx.snake.len() <= 3 {
            return ConsumeResult::GameOver;
        }

        // 触发受伤动画
        ctx.damage_state.active = true;
        ctx.damage_state.phase = DamagePhase::Flashing;
        ctx.damage_state.flash_timer = 0.0;
        ctx.damage_state.flash_count = 0;
        ctx.damage_state.crumble_timer = 0.0;
        ctx.damage_state.crumble_index = 0;
        ctx.damage_state.tail_to_remove = vec![
            ctx.snake[ctx.snake.len() - 1],
            ctx.snake[ctx.snake.len() - 2],
        ];

        ConsumeResult::ResetCombo
    }
}
