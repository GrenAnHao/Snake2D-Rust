//! 蛇蛋果实
//!
//! 触发后生成一条 AI 蛇，增加游戏挑战性。

use macroquad::prelude::*;
use ::rand::Rng;
use crate::constants::CELL;
use crate::fruits::{FruitConfig, FruitCategory, FruitContext, FruitBehavior, ConsumeResult};

/// 蛇蛋果实
pub struct SnakeEggFruit {
    config: FruitConfig,
}

impl SnakeEggFruit {
    pub fn new() -> Self {
        SnakeEggFruit {
            config: FruitConfig {
                id: "snake_egg",
                name: "蛇蛋",
                category: FruitCategory::Special,
                color: Color::new(0.95, 0.9, 0.8, 1.0), // 蛋壳色
                lifetime: 15.0,
                spawn_weight: 50, // 较高权重，增加生成概率
                unlock_length: 5,
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for SnakeEggFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SnakeEggFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 被吃掉 = 阻止孵化，只加分
        *ctx.score += 5;
        
        // 生成蛋壳碎片粒子效果
        let head = ctx.snake[0];
        for _ in 0..10 {
            let lifetime = 0.6;
            ctx.particles.push(crate::types::Particle {
                pos: vec2(
                    head.x as f32 * CELL + CELL / 2.0,
                    head.y as f32 * CELL + CELL / 2.0,
                ),
                vel: vec2(
                    ctx.rng.gen_range(-40.0..40.0),
                    ctx.rng.gen_range(-40.0..40.0),
                ),
                color: Color::new(0.95, 0.9, 0.8, 1.0),
                lifetime,
                max_lifetime: lifetime,
                size: ctx.rng.gen_range(3.0..6.0),
            });
        }
        
        // 被吃掉不生成 AI 蛇
        ConsumeResult::AddScore(0) // 分数已经加过了
    }

    /// 蛇蛋过期时孵化 AI 蛇
    ///
    /// 这是声明式架构的核心：所有蛇蛋相关逻辑都在这个文件中，
    /// 主循环不需要知道蛇蛋的存在。
    fn on_expire(&self, ctx: &mut FruitContext) {
        // 直接生成 AI 蛇 - 不需要事件中转
        ctx.ai_manager.spawn_snake(ctx.snake, ctx.rng);
        
        // 生成孵化粒子效果
        let pos = ctx.fruit_pos;
        for _ in 0..15 {
            let lifetime = 0.8;
            ctx.particles.push(crate::types::Particle {
                pos: vec2(
                    pos.x as f32 * CELL + CELL / 2.0,
                    pos.y as f32 * CELL + CELL / 2.0,
                ),
                vel: vec2(
                    ctx.rng.gen_range(-60.0..60.0),
                    ctx.rng.gen_range(-60.0..60.0),
                ),
                color: Color::new(0.95, 0.9, 0.8, 1.0), // 蛋壳色
                lifetime,
                max_lifetime: lifetime,
                size: ctx.rng.gen_range(4.0..8.0),
            });
        }
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        // 蛋形外观
        let pulse = (time * 2.0).sin() * 0.1 + 0.9;
        let color = Color::new(
            self.config.color.r * pulse,
            self.config.color.g * pulse,
            self.config.color.b * pulse,
            1.0,
        );
        
        // 蛋的主体（椭圆形）
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        let rx = CELL / 2.0 - 2.0;
        let ry = CELL / 2.0 - 1.0;
        
        // 用多个矩形近似椭圆
        for i in 0..8 {
            let angle = i as f32 * std::f32::consts::PI / 4.0;
            let px = cx + angle.cos() * rx * 0.7;
            let py = cy + angle.sin() * ry * 0.7;
            draw_rectangle(px - 3.0, py - 3.0, 6.0, 6.0, color);
        }
        
        // 蛋的中心
        draw_rectangle(x + 4.0, y + 3.0, CELL - 8.0, CELL - 6.0, color);
        
        // 裂纹效果
        let crack_alpha = ((time * 3.0).sin() * 0.5 + 0.5) * 0.6;
        let crack_color = Color::new(0.3, 0.2, 0.1, crack_alpha);
        draw_line(x + 8.0, y + 6.0, x + 12.0, y + 10.0, 1.0, crack_color);
        draw_line(x + 12.0, y + 10.0, x + 10.0, y + 14.0, 1.0, crack_color);
        
        // 高光
        draw_rectangle(x + 6.0, y + 5.0, 3.0, 3.0, Color::new(1.0, 1.0, 1.0, 0.4));
    }
}
