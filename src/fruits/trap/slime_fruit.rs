//! 粘液果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, SLIME_DURATION};

/// 粘液果实
///
/// 绿色果实，吃到后蛇身粘稠5秒，有拖拽抖动
pub struct SlimeFruit {
    config: FruitConfig,
}

impl SlimeFruit {
    pub fn new() -> Self {
        SlimeFruit {
            config: FruitConfig {
                id: "slime",
                name: "粘液果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.3, g: 0.8, b: 0.3, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 15,
                unlock_length: 0,
                immune_to_buffs: true,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for SlimeFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SlimeFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活粘液
        ctx.buff_state.slime_active = true;
        ctx.buff_state.slime_timer = SLIME_DURATION;

        ConsumeResult::ResetCombo
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let _cy = y + CELL / 2.0;
        
        // 绿色粘液背景
        let bg_color = Color::new(0.2, 0.7, 0.2, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 粘液滴落效果
        let drip_color = Color::new(0.4, 0.9, 0.4, 0.9);
        let drip_offset = (time * 2.0).sin() * 2.0;
        
        // 主体水滴形状
        draw_rectangle(cx - 4.0, y + 4.0, 8.0, 10.0, drip_color);
        draw_rectangle(cx - 3.0, y + 3.0, 6.0, 12.0, drip_color);
        draw_rectangle(cx - 2.0, y + 14.0 + drip_offset, 4.0, 3.0, drip_color);
        
        // 高光
        let highlight = Color::new(0.7, 1.0, 0.7, 0.6);
        draw_rectangle(cx - 2.0, y + 5.0, 2.0, 3.0, highlight);
        
        // 小气泡
        let bubble_alpha = (time * 4.0).sin() * 0.3 + 0.5;
        let bubble_color = Color::new(0.6, 1.0, 0.6, bubble_alpha);
        draw_rectangle(x + 4.0, y + 8.0, 3.0, 3.0, bubble_color);
        draw_rectangle(x + CELL - 7.0, y + 6.0, 2.0, 2.0, bubble_color);
        
        // 底部滴落
        let drip2_offset = (time * 3.0 + 1.0).sin() * 1.5;
        draw_rectangle(x + 5.0, y + CELL - 3.0 + drip2_offset, 3.0, 3.0, drip_color);
    }
}
