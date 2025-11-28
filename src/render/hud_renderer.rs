//! HUD 渲染模块
//!
//! 提供分数、Combo、Buff状态等 HUD 元素的绘制

use macroquad::prelude::*;
use crate::constants::{GRID_W, GRID_H, CELL};
use crate::types::{ComboState, BuffState};

/// 绘制 HUD（分数、最高分、Combo、Buff状态）
pub fn draw_hud(score: u32, high_score: u32, combo: &ComboState, buff: &BuffState) {
    // 分数
    draw_text(&format!("Score: {}", score), 8.0, 18.0, 24.0, WHITE);
    draw_text(&format!("Best: {}", high_score), 140.0, 18.0, 24.0, YELLOW);

    // Combo
    if combo.count > 1 && combo.display_timer > 0.0 {
        let combo_color = match combo.count {
            2 => YELLOW,
            3 => ORANGE,
            4 => Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 },
            _ => RED,
        };
        let combo_text = format!("COMBO x{}!", combo.count);
        let size = 28.0 + (combo.count as f32 * 2.0).min(10.0);
        draw_text(&combo_text, 280.0, 18.0, size, combo_color);
    }

    // Buff 状态
    let mut buff_y = 60.0;
    if buff.shield_active {
        draw_text(&format!("Shield: {:.1}s", buff.shield_timer), 8.0, buff_y, 18.0, GOLD);
        buff_y += 18.0;
    }
    if buff.speed_active {
        draw_text(&format!("Speed: {:.1}s", buff.speed_timer), 8.0, buff_y, 18.0, BLUE);
        buff_y += 18.0;
    }
    if buff.ghost_active {
        draw_text(&format!("Ghost: {:.1}s", buff.ghost_timer), 8.0, buff_y, 18.0, WHITE);
        buff_y += 18.0;
    }
    if buff.frozen {
        draw_text(&format!("FROZEN: {:.1}s", buff.freeze_timer), 8.0, buff_y, 18.0, SKYBLUE);
    }

    // 操作提示
    draw_text("[Arrows] Move  [Space] Pause  [Enter/R] Restart  [W] Wrap", 8.0, 40.0, 20.0, LIGHTGRAY);
}

/// 绘制覆盖层（暂停、游戏结束）
pub fn draw_overlay(title: &str, subtitle: &str) {
    let w = GRID_W as f32 * CELL;
    let h = GRID_H as f32 * CELL;

    // 半透明背景
    draw_rectangle(0.0, 0.0, w, h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 });

    // 标题
    let t = measure_text(title, None, 48, 1.0);
    draw_text(title, (w - t.width) * 0.5, h * 0.5 - 10.0, 48.0, WHITE);

    // 副标题
    let s = measure_text(subtitle, None, 24, 1.0);
    draw_text(subtitle, (w - s.width) * 0.5, h * 0.5 + 30.0, 24.0, LIGHTGRAY);
}
