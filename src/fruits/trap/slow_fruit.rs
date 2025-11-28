//! 减速果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::{TRAP_LIFETIME, SLOW_DURATION};

/// 减速果实
///
/// 橙色果实，吃到后移动速度变慢5秒，蛇身抽搐
pub struct SlowFruit {
    config: FruitConfig,
}

impl SlowFruit {
    pub fn new() -> Self {
        SlowFruit {
            config: FruitConfig {
                id: "slow",
                name: "减速果实",
                category: FruitCategory::Trap,
                color: Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 20,
                unlock_length: 0,
                immune_to_buffs: true,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for SlowFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for SlowFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 激活减速
        ctx.buff_state.slow_active = true;
        ctx.buff_state.slow_timer = SLOW_DURATION;

        ConsumeResult::ResetCombo
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let cy = y + CELL / 2.0;
        
        // 橙色背景
        let bg_color = Color::new(1.0, 0.5, 0.1, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 蜗牛图案
        let shell_color = Color::new(0.6, 0.3, 0.1, 1.0);
        let body_color = Color::new(0.9, 0.8, 0.6, 1.0);
        
        // 蜗牛壳（螺旋）
        draw_rectangle(cx - 2.0, cy - 4.0, 8.0, 8.0, shell_color);
        draw_rectangle(cx, cy - 2.0, 4.0, 4.0, Color::new(0.8, 0.5, 0.2, 1.0));
        draw_rectangle(cx + 1.0, cy - 1.0, 2.0, 2.0, Color::new(0.5, 0.25, 0.1, 1.0));
        
        // 蜗牛身体
        draw_rectangle(x + 3.0, cy + 2.0, 10.0, 4.0, body_color);
        draw_rectangle(x + 2.0, cy + 3.0, 3.0, 2.0, body_color);
        
        // 触角
        draw_rectangle(x + 4.0, cy - 1.0, 2.0, 4.0, body_color);
        draw_rectangle(x + 3.0, cy - 2.0, 2.0, 2.0, body_color);
        
        // 眼睛
        draw_rectangle(x + 3.0, cy - 2.0, 1.0, 1.0, Color::new(0.1, 0.1, 0.1, 1.0));
        
        // 脉动效果
        let pulse = (time * 2.0).sin() * 0.2 + 0.8;
        let glow = Color::new(1.0, 0.6, 0.2, (1.0 - pulse) * 0.3);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
