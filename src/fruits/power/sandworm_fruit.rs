//! 沙虫果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SANDWORM_FRUIT_LIFETIME, CELL};
use crate::types::SandwormPhase;

/// 沙虫果实
///
/// 沙色果实，触发沙虫模式，填满整个游戏盘
pub struct SandwormFruit {
    config: FruitConfig,
}

impl SandwormFruit {
    pub fn new() -> Self {
        SandwormFruit {
            config: FruitConfig {
                id: "sandworm",
                name: "沙虫果实",
                category: FruitCategory::Power,
                color: Color { r: 0.76, g: 0.60, b: 0.42, a: 1.0 },
                lifetime: SANDWORM_FRUIT_LIFETIME,
                spawn_weight: 1,  // 最低权重，非常稀有
                unlock_length: 15, // 需要更长的蛇才能解锁
                immune_to_buffs: false,
            },
        }
    }
}

impl Default for SandwormFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SandwormFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 激活沙虫模式
        ctx.buff_state.sandworm_active = true;
        ctx.buff_state.sandworm_phase = SandwormPhase::Flashing;
        ctx.buff_state.sandworm_phase_timer = 0.0;
        ctx.buff_state.sandworm_transform_index = 0;
        ctx.buff_state.sandworm_original_snake = ctx.snake.clone();
        ctx.buff_state.sandworm_original_dir = *ctx.dir;
        ctx.buff_state.sandworm_path = vec![];
        ctx.buff_state.sandworm_index = 0;
        ctx.buff_state.sandworm_tick = 0.0;

        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        // 沙虫果实使用斑驳纹理
        let base_color = self.config.color;
        draw_rectangle(x, y, CELL, CELL, base_color);

        // 斑点
        let spot_count = 5;
        let spot_size = CELL * 0.22;
        let pos_x = (x / CELL) as i32;
        let pos_y = (y / CELL) as i32;

        for i in 0..spot_count {
            let hash1 = ((pos_x * 12347 + pos_y * 98765 + i * 53) % 100) as f32 / 100.0;
            let hash2 = ((pos_x * 98765 + pos_y * 12347 + i * 71) % 100) as f32 / 100.0;
            let hash3 = ((pos_x * 53 + pos_y * 29 + i * 12347) % 100) as f32 / 100.0;

            let spot_x = x + hash1 * (CELL - spot_size);
            let spot_y = y + hash2 * (CELL - spot_size);

            let dark_factor = 0.25 + hash3 * 0.35;
            let spot_color = Color {
                r: base_color.r * dark_factor,
                g: base_color.g * dark_factor,
                b: base_color.b * dark_factor,
                a: 0.75 + (time * 4.0 + hash1 * 8.0).sin() * 0.15,
            };

            if hash3 < 0.7 {
                draw_rectangle(
                    spot_x,
                    spot_y,
                    spot_size * (0.6 + hash3 * 0.4),
                    spot_size * (0.6 + hash1 * 0.4),
                    spot_color,
                );
            }
        }

        // 脉动边框
        let pulse = (time * 3.0).sin() * 0.3 + 0.7;
        let border_color = Color {
            r: base_color.r * 0.6,
            g: base_color.g * 0.6,
            b: base_color.b * 0.6,
            a: 0.5 * pulse,
        };
        draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);
    }
}
