//! 红十字恢复果实
//!
//! 瞬间清除所有 Debuff 效果。

use macroquad::prelude::*;
use ::rand::Rng;
use crate::constants::CELL;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::types::Particle;

/// 红十字恢复果实
pub struct HealFruit {
    config: FruitConfig,
}

impl HealFruit {
    pub fn new() -> Self {
        HealFruit {
            config: FruitConfig {
                id: "heal",
                name: "恢复",
                category: FruitCategory::Power,
                color: Color::new(1.0, 1.0, 1.0, 1.0), // 白色
                lifetime: 10.0,
                spawn_weight: 5, // 较低概率（稀有）
                unlock_length: 8, // 蛇长度>=8才生成
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for HealFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for HealFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 清除所有 Debuff
        ctx.buff_state.clear_all_debuffs();
        
        // 生成治愈粒子效果（绿色/白色）
        let head = ctx.snake[0];
        let center = vec2(
            head.x as f32 * CELL + CELL / 2.0,
            head.y as f32 * CELL + CELL / 2.0,
        );
        
        for _ in 0..20 {
            let angle = ctx.rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = ctx.rng.gen_range(60.0..120.0);
            let lifetime = ctx.rng.gen_range(0.4..0.8);
            
            // 绿色和白色粒子交替
            let color = if ctx.rng.gen_bool(0.5) {
                Color::new(0.3, 1.0, 0.3, 1.0) // 绿色
            } else {
                Color::new(1.0, 1.0, 1.0, 1.0) // 白色
            };
            
            ctx.particles.push(Particle {
                pos: center,
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                color,
                lifetime,
                max_lifetime: lifetime,
                size: ctx.rng.gen_range(3.0..6.0),
            });
        }
        
        // 加分
        *ctx.score += 20;
        
        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 白色背景（圆角矩形）
        let bg_color = Color::new(1.0, 1.0, 1.0, 1.0);
        draw_rectangle(x + 2.0, y + 4.0, CELL - 4.0, CELL - 8.0, bg_color);
        draw_rectangle(x + 4.0, y + 2.0, CELL - 8.0, CELL - 4.0, bg_color);
        
        // 红色十字
        let cross_color = Color::new(0.9, 0.1, 0.1, 1.0);
        let cross_width = 4.0;
        let cross_len = CELL - 8.0;
        
        // 横条
        draw_rectangle(cx - cross_len / 2.0, cy - cross_width / 2.0, cross_len, cross_width, cross_color);
        // 竖条
        draw_rectangle(cx - cross_width / 2.0, cy - cross_len / 2.0, cross_width, cross_len, cross_color);
        
        // 脉动光晕效果
        let pulse = (time * 3.0).sin() * 0.3 + 0.7;
        let glow_color = Color::new(1.0, 1.0, 1.0, pulse * 0.3);
        draw_rectangle(x, y, CELL, CELL, glow_color);
    }
}
