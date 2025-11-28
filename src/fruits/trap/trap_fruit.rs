//! 陷阱果实实现（紫色，减蛇身）

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};
use crate::constants::TRAP_LIFETIME;
use crate::types::DamagePhase;

/// 陷阱果实
///
/// 紫色果实，吃到后蛇身减少2格，触发受伤动画
pub struct TrapFruit {
    config: FruitConfig,
}

impl TrapFruit {
    pub fn new() -> Self {
        TrapFruit {
            config: FruitConfig {
                id: "trap",
                name: "陷阱果实",
                category: FruitCategory::Trap,
                color: Color { r: 0.6, g: 0.2, b: 0.8, a: 1.0 },
                lifetime: TRAP_LIFETIME,
                spawn_weight: 35,
                unlock_length: 0,
                immune_to_buffs: true,
                weight_growth: 0, // 不增长
            },
        }
    }
}

impl Default for TrapFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for TrapFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 检查免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::ResetCombo;
        }

        // 蛇太短直接死亡
        if ctx.snake.len() <= 3 {
            return ConsumeResult::GameOver;
        }

        // 触发受伤动画
        ctx.damage_state.active = true;
        ctx.damage_state.phase = DamagePhase::Flashing;
        ctx.damage_state.flash_timer = 0.0;
        ctx.damage_state.flash_count = 0;
        ctx.damage_state.crumble_timer = 0.0;
        ctx.damage_state.crumble_index = 0;
        ctx.damage_state.tail_to_remove = vec![
            ctx.snake[ctx.snake.len() - 1],
            ctx.snake[ctx.snake.len() - 2],
        ];

        ConsumeResult::ResetCombo
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        use crate::constants::CELL;
        let cx = x + CELL / 2.0;
        let _cy = y + CELL / 2.0;
        
        // 紫色背景
        let bg_color = Color::new(0.5, 0.15, 0.6, 1.0);
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bg_color);
        
        // 骷髅头（简化版）
        let skull_color = Color::new(0.9, 0.85, 0.8, 1.0);
        // 头部
        draw_rectangle(cx - 5.0, y + 3.0, 10.0, 8.0, skull_color);
        draw_rectangle(cx - 4.0, y + 2.0, 8.0, 10.0, skull_color);
        // 眼睛
        let eye_color = Color::new(0.1, 0.0, 0.1, 1.0);
        draw_rectangle(cx - 4.0, y + 5.0, 3.0, 3.0, eye_color);
        draw_rectangle(cx + 1.0, y + 5.0, 3.0, 3.0, eye_color);
        // 鼻子
        draw_rectangle(cx - 1.0, y + 9.0, 2.0, 2.0, eye_color);
        // 下巴/牙齿
        draw_rectangle(cx - 4.0, y + 12.0, 8.0, 4.0, skull_color);
        let teeth_color = Color::new(0.2, 0.1, 0.2, 1.0);
        draw_rectangle(cx - 3.0, y + 13.0, 2.0, 2.0, teeth_color);
        draw_rectangle(cx + 1.0, y + 13.0, 2.0, 2.0, teeth_color);
        
        // 闪烁效果
        let flash = (time * 4.0).sin() * 0.2 + 0.8;
        let glow = Color::new(0.8, 0.2, 1.0, (1.0 - flash) * 0.4);
        draw_rectangle(x, y, CELL, CELL, glow);
    }
}
