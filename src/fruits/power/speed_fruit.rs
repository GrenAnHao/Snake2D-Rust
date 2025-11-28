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
                weight_growth: 0, // 不增长
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

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 蓝色背景
        let bg_color = Color::new(0.1, 0.5, 0.9, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 闪电图案
        let bolt_color = Color::new(1.0, 1.0, 0.3, 1.0);
        // 闪电主体（Z字形）
        draw_rectangle(cx - 1.0, y + 2.0, 6.0, 3.0, bolt_color);
        draw_rectangle(cx + 2.0, y + 4.0, 3.0, 4.0, bolt_color);
        draw_rectangle(cx - 3.0, y + 7.0, 8.0, 3.0, bolt_color);
        draw_rectangle(cx - 3.0, y + 9.0, 3.0, 4.0, bolt_color);
        draw_rectangle(cx - 5.0, y + 12.0, 6.0, 3.0, bolt_color);
        draw_rectangle(cx - 2.0, y + 14.0, 3.0, 3.0, bolt_color);
        
        // 高光
        let highlight = Color::new(1.0, 1.0, 1.0, 0.6);
        draw_rectangle(cx, y + 3.0, 2.0, 2.0, highlight);
        
        // 速度线效果
        let line_alpha = (time * 8.0).sin() * 0.4 + 0.4;
        let line_color = Color::new(0.5, 0.8, 1.0, line_alpha);
        draw_rectangle(x + 1.0, cy - 1.0, 3.0, 2.0, line_color);
        draw_rectangle(x + 1.0, cy + 3.0, 4.0, 1.0, line_color);
        
        // 脉动效果
        let pulse = (time * 5.0).sin() * 0.2 + 0.8;
        let glow = Color::new(0.3, 0.7, 1.0, (1.0 - pulse) * 0.3);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
