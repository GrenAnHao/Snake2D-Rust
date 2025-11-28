//! 逆转果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};

/// 逆转果实
///
/// 粉色果实，蛇头尾互换
pub struct ReverseFruit {
    config: FruitConfig,
}

impl ReverseFruit {
    pub fn new() -> Self {
        ReverseFruit {
            config: FruitConfig {
                id: "reverse",
                name: "逆转果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 0.4, b: 0.7, a: 1.0 },
                lifetime: 5.0,
                spawn_weight: 15,
                unlock_length: 10,
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for ReverseFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for ReverseFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 逆转蛇身
        ctx.snake.reverse();

        // 更新方向为新头部的方向
        if ctx.snake.len() > 1 {
            let new_head = ctx.snake[0];
            let second = ctx.snake[1];
            *ctx.dir = ivec2(new_head.x - second.x, new_head.y - second.y);
        }

        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 粉色背景
        let bg_color = Color::new(1.0, 0.3, 0.6, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 双向箭头图案
        let arrow_color = Color::new(1.0, 1.0, 1.0, 0.95);
        
        // 左箭头
        draw_rectangle(x + 3.0, cy - 1.0, 6.0, 2.0, arrow_color);
        draw_rectangle(x + 3.0, cy - 3.0, 2.0, 2.0, arrow_color);
        draw_rectangle(x + 3.0, cy + 1.0, 2.0, 2.0, arrow_color);
        draw_rectangle(x + 2.0, cy - 1.0, 2.0, 2.0, arrow_color);
        
        // 右箭头
        draw_rectangle(cx + 1.0, cy - 1.0, 6.0, 2.0, arrow_color);
        draw_rectangle(x + CELL - 5.0, cy - 3.0, 2.0, 2.0, arrow_color);
        draw_rectangle(x + CELL - 5.0, cy + 1.0, 2.0, 2.0, arrow_color);
        draw_rectangle(x + CELL - 4.0, cy - 1.0, 2.0, 2.0, arrow_color);
        
        // 中间分隔
        let sep_color = Color::new(0.8, 0.2, 0.5, 1.0);
        draw_rectangle(cx - 1.0, cy - 4.0, 2.0, 8.0, sep_color);
        
        // 旋转效果
        let rotation_alpha = (time * 4.0).sin() * 0.3 + 0.5;
        let glow = Color::new(1.0, 0.5, 0.8, rotation_alpha * 0.3);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
