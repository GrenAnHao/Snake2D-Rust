//! 护盾果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SHIELD_FRUIT_LIFETIME, SHIELD_DURATION};

/// 护盾果实
///
/// 金色果实，10秒无敌护盾
pub struct ShieldFruit {
    config: FruitConfig,
}

impl ShieldFruit {
    pub fn new() -> Self {
        ShieldFruit {
            config: FruitConfig {
                id: "shield",
                name: "护盾果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 },
                lifetime: SHIELD_FRUIT_LIFETIME,
                spawn_weight: 30,
                unlock_length: 10,
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for ShieldFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for ShieldFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        ctx.buff_state.shield_active = true;
        ctx.buff_state.shield_timer = SHIELD_DURATION;
        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let _cy = y + CELL / 2.0;
        
        // 金色背景
        let bg_color = Color::new(0.9, 0.75, 0.0, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 盾牌形状
        let shield_color = Color::new(1.0, 0.9, 0.3, 1.0);
        // 盾牌上半部分（矩形）
        draw_rectangle(cx - 6.0, y + 3.0, 12.0, 8.0, shield_color);
        // 盾牌下半部分（三角形用矩形近似）
        draw_rectangle(cx - 5.0, y + 11.0, 10.0, 3.0, shield_color);
        draw_rectangle(cx - 4.0, y + 14.0, 8.0, 2.0, shield_color);
        draw_rectangle(cx - 2.0, y + 16.0, 4.0, 2.0, shield_color);
        
        // 盾牌中心装饰（星形）
        let star_color = Color::new(0.8, 0.6, 0.0, 1.0);
        draw_rectangle(cx - 2.0, y + 6.0, 4.0, 6.0, star_color);
        draw_rectangle(cx - 4.0, y + 8.0, 8.0, 2.0, star_color);
        
        // 高光
        let highlight = Color::new(1.0, 1.0, 0.8, 0.5);
        draw_rectangle(cx - 4.0, y + 4.0, 3.0, 4.0, highlight);
        
        // 脉动光晕
        let pulse = (time * 3.0).sin() * 0.3 + 0.7;
        let glow = Color::new(1.0, 0.9, 0.4, pulse * 0.4);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
