//! 果实渲染模块
//!
//! 提供果实的绘制功能

use macroquad::prelude::*;
use crate::constants::CELL;
use crate::types::Fruit;
use crate::fruits::FruitRegistry;

/// 绘制所有果实
pub fn draw_fruits(fruits: &[Fruit], registry: &FruitRegistry, game_time: f32) {
    for fruit in fruits {
        let x = fruit.pos.x as f32 * CELL;
        let y = fruit.pos.y as f32 * CELL;

        // 使用 FruitBehavior::render 绘制
        if let Some(behavior) = registry.get(fruit.type_id) {
            behavior.render(x, y, game_time);
        } else {
            // 回退：绘制默认方块
            draw_rectangle(x, y, CELL, CELL, RED);
        }

        // 有时限的果实显示进度条
        if fruit.lifetime > 0.0 {
            let progress = fruit.remaining_ratio(game_time);
            draw_progress_bar(fruit.pos, progress);
        }
    }
}

/// 绘制普通食物
pub fn draw_food(pos: IVec2) {
    let x = pos.x as f32 * CELL;
    let y = pos.y as f32 * CELL;
    draw_rectangle(x, y, CELL, CELL, RED);
}

/// 绘制进度条
pub fn draw_progress_bar(pos: IVec2, progress: f32) {
    let x = pos.x as f32 * CELL;
    let y = pos.y as f32 * CELL - 4.0;
    let w = CELL;
    let h = 3.0;

    // 背景
    draw_rectangle(x, y, w, h, DARKGRAY);
    // 进度
    let prog_color = if progress > 0.3 { YELLOW } else { RED };
    draw_rectangle(x, y, w * progress, h, prog_color);
}

/// 绘制幸运方块果实
pub fn draw_lucky_fruit(x: f32, y: f32, time: f32) {
    // 礼盒底色（黄色）
    let box_color = Color { r: 1.0, g: 0.85, b: 0.2, a: 1.0 };
    draw_rectangle(x, y, CELL, CELL, box_color);

    // 礼盒边框（深黄色）
    let border_color = Color { r: 0.8, g: 0.6, b: 0.1, a: 1.0 };
    draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);

    // 红色丝带（十字形）
    let ribbon_color = Color { r: 0.9, g: 0.2, b: 0.2, a: 1.0 };
    let ribbon_width = 4.0;
    draw_rectangle(x, y + CELL / 2.0 - ribbon_width / 2.0, CELL, ribbon_width, ribbon_color);
    draw_rectangle(x + CELL / 2.0 - ribbon_width / 2.0, y, ribbon_width, CELL, ribbon_color);

    // 问号（脉动效果）
    let pulse = (time * 4.0).sin() * 0.2 + 0.8;
    let question_color = Color { r: 1.0, g: 1.0, b: 1.0, a: pulse };

    let qx = x + CELL / 2.0;
    let qy = y + CELL / 2.0;
    let qs = 3.0;

    draw_rectangle(qx - qs, qy - 6.0, qs * 2.0, qs, question_color);
    draw_rectangle(qx + qs - 1.0, qy - 5.0, qs, qs * 2.0, question_color);
    draw_rectangle(qx - 1.0, qy - 1.0, qs, qs, question_color);
    draw_rectangle(qx - qs / 2.0, qy + 4.0, qs, qs, question_color);
}

/// 绘制沙虫果实（带斑驳纹理）
pub fn draw_sandworm_fruit(x: f32, y: f32, time: f32, pos_x: i32, pos_y: i32) {
    let base_color = sandworm_body_color();

    draw_rectangle(x, y, CELL, CELL, base_color);

    let spot_count = 5;
    let spot_size = CELL * 0.22;

    for i in 0..spot_count {
        let hash1 = ((pos_x * 12347 + pos_y * 98765 + i * 53) % 100) as f32 / 100.0;
        let hash2 = ((pos_x * 98765 + pos_y * 12347 + i * 71) % 100) as f32 / 100.0;
        let hash3 = ((pos_x * 53 + pos_y * 29 + i * 12347) % 100) as f32 / 100.0;

        let spot_x = x + hash1 * (CELL - spot_size);
        let spot_y = y + hash2 * (CELL - spot_size);

        let dark_factor = 0.25 + hash3 * 0.35;
        let spot_color = Color {
            r: base_color.r * dark_factor,
            g: base_color.g * dark_factor,
            b: base_color.b * dark_factor,
            a: 0.75 + (time * 4.0 + hash1 * 8.0).sin() * 0.15,
        };

        if hash3 < 0.7 {
            draw_rectangle(spot_x, spot_y, spot_size * (0.6 + hash3 * 0.4), spot_size * (0.6 + hash1 * 0.4), spot_color);
        }
    }

    // 脉动边框
    let pulse = (time * 3.0).sin() * 0.3 + 0.7;
    let border_color = Color {
        r: base_color.r * 0.6,
        g: base_color.g * 0.6,
        b: base_color.b * 0.6,
        a: 0.5 * pulse,
    };
    let border_width = 2.0;
    draw_rectangle(x, y, CELL, border_width, border_color);
    draw_rectangle(x, y + CELL - border_width, CELL, border_width, border_color);
    draw_rectangle(x, y, border_width, CELL, border_color);
    draw_rectangle(x + CELL - border_width, y, border_width, CELL, border_color);
}

/// 沙虫身体颜色
pub fn sandworm_body_color() -> Color {
    Color { r: 0.76, g: 0.60, b: 0.42, a: 1.0 }
}

/// 沙虫头部颜色
pub fn sandworm_head_color() -> Color {
    Color { r: 0.55, g: 0.40, b: 0.25, a: 1.0 }
}
