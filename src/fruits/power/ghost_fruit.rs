//! 幽灵果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{SHIELD_FRUIT_LIFETIME, GHOST_DURATION};

/// 幽灵果实
///
/// 白色半透明果实，5秒可穿墙穿身
pub struct GhostFruit {
    config: FruitConfig,
}

impl GhostFruit {
    pub fn new() -> Self {
        GhostFruit {
            config: FruitConfig {
                id: "ghost",
                name: "幽灵果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 },
                lifetime: SHIELD_FRUIT_LIFETIME,
                spawn_weight: 20,
                unlock_length: 10,
                immune_to_buffs: false,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for GhostFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for GhostFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        ctx.buff_state.ghost_active = true;
        ctx.buff_state.ghost_timer = GHOST_DURATION;
        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let _cy = y + CELL / 2.0;
        
        // 半透明白色背景
        let bg_alpha = 0.5 + (time * 2.0).sin() * 0.2;
        let bg_color = Color::new(0.9, 0.9, 1.0, bg_alpha);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 幽灵形状
        let ghost_alpha = 0.7 + (time * 3.0).sin() * 0.2;
        let ghost_color = Color::new(1.0, 1.0, 1.0, ghost_alpha);
        
        // 幽灵头部（圆形）
        draw_rectangle(cx - 5.0, y + 3.0, 10.0, 8.0, ghost_color);
        draw_rectangle(cx - 4.0, y + 2.0, 8.0, 10.0, ghost_color);
        
        // 幽灵身体
        draw_rectangle(cx - 5.0, y + 10.0, 10.0, 6.0, ghost_color);
        
        // 幽灵底部波浪
        let wave_offset = (time * 4.0).sin() * 1.0;
        draw_rectangle(cx - 5.0, y + 15.0 + wave_offset, 3.0, 3.0, ghost_color);
        draw_rectangle(cx - 1.0, y + 16.0 - wave_offset, 3.0, 3.0, ghost_color);
        draw_rectangle(cx + 3.0, y + 15.0 + wave_offset, 3.0, 3.0, ghost_color);
        
        // 眼睛
        let eye_color = Color::new(0.2, 0.2, 0.3, 0.9);
        draw_rectangle(cx - 4.0, y + 6.0, 3.0, 4.0, eye_color);
        draw_rectangle(cx + 1.0, y + 6.0, 3.0, 4.0, eye_color);
        
        // 眼睛高光
        let eye_highlight = Color::new(1.0, 1.0, 1.0, 0.8);
        draw_rectangle(cx - 3.0, y + 7.0, 1.0, 1.0, eye_highlight);
        draw_rectangle(cx + 2.0, y + 7.0, 1.0, 1.0, eye_highlight);
        
        // 飘动效果
        let float_offset = (time * 2.0).sin() * 2.0;
        let glow = Color::new(0.8, 0.8, 1.0, 0.2);
        draw_rectangle(x - 1.0, y - 1.0 + float_offset, CELL + 2.0, CELL + 2.0, glow);
    }
}
