//! 冰冻果实实现

use macroquad::prelude::*;
use ::rand::Rng;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, FREEZE_DURATION, CELL};
use crate::types::Particle;

/// 冰冻果实
///
/// 浅蓝色果实，吃到后冻住2秒无法移动
pub struct FreezeFruit {
    config: FruitConfig,
}

impl FreezeFruit {
    pub fn new() -> Self {
        FreezeFruit {
            config: FruitConfig {
                id: "freeze",
                name: "冰冻果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.6, g: 0.9, b: 1.0, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 15,
                unlock_length: 0,
                immune_to_buffs: true,
            },
        }
    }

    /// 生成冰冻粒子
    fn spawn_freeze_particles(particles: &mut Vec<Particle>, snake: &[IVec2], rng: &mut impl Rng) {
        let ice_colors = [
            Color { r: 0.7, g: 0.9, b: 1.0, a: 1.0 },
            Color { r: 0.5, g: 0.8, b: 1.0, a: 1.0 },
            Color { r: 0.9, g: 0.95, b: 1.0, a: 1.0 },
            Color { r: 0.6, g: 0.85, b: 0.95, a: 1.0 },
        ];

        for seg in snake {
            let center = vec2(
                seg.x as f32 * CELL + CELL / 2.0,
                seg.y as f32 * CELL + CELL / 2.0,
            );

            for _ in 0..rng.gen_range(5..8) {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(20.0..80.0);
                let vel = vec2(angle.cos() * speed, angle.sin() * speed - 30.0);
                let color = ice_colors[rng.gen_range(0..ice_colors.len())];
                let lifetime = rng.gen_range(0.5..1.2);

                particles.push(Particle {
                    pos: center,
                    vel,
                    color,
                    lifetime,
                    max_lifetime: lifetime,
                    size: rng.gen_range(2.0..5.0),
                });
            }
        }
    }
}

impl Default for FreezeFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for FreezeFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活冰冻
        ctx.buff_state.frozen = true;
        ctx.buff_state.freeze_timer = FREEZE_DURATION;

        // 生成冰冻粒子
        Self::spawn_freeze_particles(ctx.particles, ctx.snake, ctx.rng);

        ConsumeResult::ResetCombo
    }
}
