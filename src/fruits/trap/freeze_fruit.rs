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
                weight_growth: 0, // 不增长
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

    fn render(&self, x: f32, y: f32, time: f32) {
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 浅蓝色背景
        let bg_color = Color::new(0.5, 0.8, 1.0, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 雪花图案（六角形）
        let snow_color = Color::new(1.0, 1.0, 1.0, 0.95);
        let arm_len = 6.0;
        
        // 6条雪花臂
        for i in 0..6 {
            let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
            let dx = angle.cos() * arm_len;
            let dy = angle.sin() * arm_len;
            draw_line(cx, cy, cx + dx, cy + dy, 2.0, snow_color);
            
            // 小分支
            let branch_len = 3.0;
            let branch_angle1 = angle + 0.5;
            let branch_angle2 = angle - 0.5;
            let mid_x = cx + dx * 0.6;
            let mid_y = cy + dy * 0.6;
            draw_line(mid_x, mid_y, 
                mid_x + branch_angle1.cos() * branch_len, 
                mid_y + branch_angle1.sin() * branch_len, 
                1.0, snow_color);
            draw_line(mid_x, mid_y, 
                mid_x + branch_angle2.cos() * branch_len, 
                mid_y + branch_angle2.sin() * branch_len, 
                1.0, snow_color);
        }
        
        // 中心点
        draw_rectangle(cx - 2.0, cy - 2.0, 4.0, 4.0, snow_color);
        
        // 闪烁效果
        let sparkle = (time * 5.0).sin() * 0.3 + 0.7;
        let glow = Color::new(0.8, 0.95, 1.0, sparkle * 0.3);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
