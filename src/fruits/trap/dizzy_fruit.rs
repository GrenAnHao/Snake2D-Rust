//! 眩晕果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, DIZZY_DURATION};

/// 眩晕果实
///
/// 黄绿色果实，吃到后方向随机偏移5秒，醉酒效果
pub struct DizzyFruit {
    config: FruitConfig,
}

impl DizzyFruit {
    pub fn new() -> Self {
        DizzyFruit {
            config: FruitConfig {
                id: "dizzy",
                name: "眩晕果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.8, g: 1.0, b: 0.3, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 15,
                unlock_length: 0,
                immune_to_buffs: true,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for DizzyFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for DizzyFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活眩晕
        ctx.buff_state.dizzy_active = true;
        ctx.buff_state.dizzy_timer = DIZZY_DURATION;

        ConsumeResult::ResetCombo
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 黄绿色背景
        let bg_color = Color::new(0.7, 0.9, 0.2, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 旋转的螺旋图案
        let spiral_color = Color::new(0.3, 0.5, 0.1, 1.0);
        let rotation = time * 3.0;
        
        // 绘制螺旋（用多个弧线段近似）
        for i in 0..12 {
            let t = i as f32 / 12.0;
            let angle = t * std::f32::consts::TAU * 1.5 + rotation;
            let r = 2.0 + t * 5.0;
            let px = cx + angle.cos() * r;
            let py = cy + angle.sin() * r;
            let size = 2.0 + t * 1.5;
            draw_rectangle(px - size / 2.0, py - size / 2.0, size, size, spiral_color);
        }
        
        // 中心眼睛（眩晕的眼睛）
        let eye_color = Color::new(0.2, 0.3, 0.0, 1.0);
        draw_rectangle(cx - 2.0, cy - 2.0, 4.0, 4.0, eye_color);
        
        // 星星效果（表示眩晕）
        let star_alpha = (time * 6.0).sin() * 0.5 + 0.5;
        let star_color = Color::new(1.0, 1.0, 0.5, star_alpha);
        draw_rectangle(x + 3.0, y + 3.0, 3.0, 3.0, star_color);
        draw_rectangle(x + CELL - 6.0, y + 3.0, 3.0, 3.0, star_color);
        draw_rectangle(x + 3.0, y + CELL - 6.0, 3.0, 3.0, star_color);
    }
}
