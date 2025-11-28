//! 幸运方块果实实现

use macroquad::prelude::*;
use ::rand::Rng;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::*;
use crate::types::{Particle, DamagePhase};

/// 幸运方块果实
///
/// 黄色礼盒带问号，随机触发增益或负面效果
pub struct LuckyFruit {
    config: FruitConfig,
}

impl LuckyFruit {
    pub fn new() -> Self {
        LuckyFruit {
            config: FruitConfig {
                id: "lucky",
                name: "幸运方块",
                category: FruitCategory::Special,
                color: Color { r: 1.0, g: 0.9, b: 0.2, a: 1.0 },
                lifetime: LUCKY_LIFETIME,
                spawn_weight: 100,
                unlock_length: 0,
                immune_to_buffs: false,
            },
        }
    }

    /// 生成开箱粒子
    fn spawn_lucky_particles(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut impl Rng) {
        let colors = [
            Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 },
            Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 },
            Color { r: 1.0, g: 1.0, b: 0.2, a: 1.0 },
            Color { r: 0.2, g: 1.0, b: 0.2, a: 1.0 },
            Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 },
            Color { r: 0.8, g: 0.2, b: 1.0, a: 1.0 },
        ];

        for _ in 0..25 {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(80.0..200.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed);
            let color = colors[rng.gen_range(0..colors.len())];
            let lifetime = rng.gen_range(0.5..1.0);

            particles.push(Particle {
                pos,
                vel,
                color,
                lifetime,
                max_lifetime: lifetime,
                size: rng.gen_range(3.0..8.0),
            });
        }
    }
}

impl Default for LuckyFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for LuckyFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 生成开箱粒子
        let center = vec2(
            ctx.fruit_pos.x as f32 * CELL + CELL / 2.0,
            ctx.fruit_pos.y as f32 * CELL + CELL / 2.0,
        );
        Self::spawn_lucky_particles(ctx.particles, center, ctx.rng);

        // 50%增益，50%负面
        let is_positive = ctx.rng.gen_bool(0.5);

        if is_positive {
            // 增益效果
            match ctx.rng.gen_range(0..5) {
                0 => {
                    ctx.buff_state.shield_active = true;
                    ctx.buff_state.shield_timer = SHIELD_DURATION;
                }
                1 => {
                    ctx.buff_state.speed_active = true;
                    ctx.buff_state.speed_timer = SPEED_DURATION;
                }
                2 => {
                    ctx.buff_state.ghost_active = true;
                    ctx.buff_state.ghost_timer = GHOST_DURATION;
                }
                3 => {
                    *ctx.score += 10;
                }
                _ => {
                    // 蛇身增长3节
                    for _ in 0..3 {
                        if let Some(tail) = ctx.snake.last().cloned() {
                            ctx.snake.push(tail);
                        }
                    }
                }
            }
            ConsumeResult::Continue
        } else {
            // 负面效果
            if ctx.buff_state.has_immunity() {
                return ConsumeResult::ResetCombo;
            }

            match ctx.rng.gen_range(0..6) {
                0 => {
                    ctx.buff_state.frozen = true;
                    ctx.buff_state.freeze_timer = FREEZE_DURATION;
                }
                1 => {
                    ctx.buff_state.slow_active = true;
                    ctx.buff_state.slow_timer = SLOW_DURATION;
                }
                2 => {
                    ctx.buff_state.dizzy_active = true;
                    ctx.buff_state.dizzy_timer = DIZZY_DURATION;
                }
                3 => {
                    ctx.buff_state.slime_active = true;
                    ctx.buff_state.slime_timer = SLIME_DURATION;
                }
                4 => {
                    // 蛇身减少
                    if ctx.snake.len() > 3 {
                        ctx.damage_state.active = true;
                        ctx.damage_state.phase = DamagePhase::Flashing;
                        ctx.damage_state.flash_timer = 0.0;
                        ctx.damage_state.flash_count = 0;
                        ctx.damage_state.tail_to_remove = vec![
                            ctx.snake[ctx.snake.len() - 1],
                            ctx.snake[ctx.snake.len() - 2],
                        ];
                    }
                }
                _ => {
                    // 生成 AI 蛇（蛇蛋效果）
                    // 设置一个特殊标记，通过 buff_state 传递
                    // pending_ai_spawn 会在主循环中检查
                    ctx.buff_state.pending_ai_spawn = true;
                }
            }
            ConsumeResult::ResetCombo
        }
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        // 礼盒底色
        let box_color = Color { r: 1.0, g: 0.85, b: 0.2, a: 1.0 };
        draw_rectangle(x, y, CELL, CELL, box_color);

        // 边框
        let border_color = Color { r: 0.8, g: 0.6, b: 0.1, a: 1.0 };
        draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);

        // 红色丝带
        let ribbon_color = Color { r: 0.9, g: 0.2, b: 0.2, a: 1.0 };
        let ribbon_width = 4.0;
        draw_rectangle(x, y + CELL / 2.0 - ribbon_width / 2.0, CELL, ribbon_width, ribbon_color);
        draw_rectangle(x + CELL / 2.0 - ribbon_width / 2.0, y, ribbon_width, CELL, ribbon_color);

        // 问号
        let pulse = (time * 4.0).sin() * 0.2 + 0.8;
        let question_color = Color { r: 1.0, g: 1.0, b: 1.0, a: pulse };
        let qx = x + CELL / 2.0;
        let qy = y + CELL / 2.0;
        let qs = 3.0;

        draw_rectangle(qx - qs, qy - 6.0, qs * 2.0, qs, question_color);
        draw_rectangle(qx + qs - 1.0, qy - 5.0, qs, qs * 2.0, question_color);
        draw_rectangle(qx - 1.0, qy - 1.0, qs, qs, question_color);
        draw_rectangle(qx - qs / 2.0, qy + 4.0, qs, qs, question_color);
    }
}
