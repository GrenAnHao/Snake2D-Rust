//! 沙虫渲染模块
//!
//! 提供沙虫变身动画的所有阶段渲染

use macroquad::prelude::*;
use crate::constants::{CELL, GRID_W, GRID_H};
use crate::types::{BuffState, SandwormPhase};
use super::fruit_renderer::{sandworm_body_color, sandworm_head_color};

/// 沙虫吞噬颜色（红色）
pub fn sandworm_consume_color() -> Color {
    Color { r: 0.8, g: 0.2, b: 0.2, a: 1.0 }
}

/// 绘制沙虫变身动画
pub fn draw_sandworm_transform(snake: &[IVec2], buff: &BuffState, time: f32) {
    match buff.sandworm_phase {
        SandwormPhase::Flashing => {
            draw_sandworm_flashing(snake, time);
        }
        SandwormPhase::Transforming => {
            draw_sandworm_transforming(snake, buff, time);
        }
        _ => {}
    }
}

/// 绘制闪烁阶段
fn draw_sandworm_flashing(snake: &[IVec2], time: f32) {
    let flash_speed = 12.0;
    let flash = ((time * flash_speed).sin() * 0.5 + 0.5).powi(2);

    for (i, seg) in snake.iter().enumerate() {
        let base_color = if i == 0 { GREEN } else { LIME };
        let color = Color {
            r: base_color.r + (1.0 - base_color.r) * flash,
            g: base_color.g + (1.0 - base_color.g) * flash,
            b: base_color.b + (1.0 - base_color.b) * flash,
            a: 1.0,
        };
        let x = seg.x as f32 * CELL;
        let y = seg.y as f32 * CELL;
        draw_rectangle(x, y, CELL, CELL, color);

        if flash > 0.5 {
            let glow = Color { r: 1.0, g: 1.0, b: 0.8, a: (flash - 0.5) * 0.6 };
            draw_rectangle(x - 2.0, y - 2.0, CELL + 4.0, CELL + 4.0, glow);
        }
    }
}

/// 绘制变色阶段
fn draw_sandworm_transforming(snake: &[IVec2], buff: &BuffState, time: f32) {
    let sand_color = sandworm_body_color();
    let transform_interval = 0.08;

    for (i, seg) in snake.iter().enumerate() {
        let x = seg.x as f32 * CELL;
        let y = seg.y as f32 * CELL;

        if i < buff.sandworm_transform_index {
            if i == 0 {
                let head_dir = if snake.len() > 1 {
                    let next = snake[1];
                    ivec2(seg.x - next.x, seg.y - next.y)
                } else {
                    buff.sandworm_original_dir
                };
                draw_sandworm_head(x, y, head_dir);
            } else {
                draw_sandworm_segment(x, y, CELL, sand_color, seg.x, seg.y, time);
            }
        } else if i == buff.sandworm_transform_index {
            let progress = buff.sandworm_phase_timer / transform_interval;
            let base = if i == 0 { GREEN } else { LIME };
            let target = if i == 0 { sandworm_head_color() } else { sand_color };
            let color = Color {
                r: base.r + (target.r - base.r) * progress,
                g: base.g + (target.g - base.g) * progress,
                b: base.b + (target.b - base.b) * progress,
                a: 1.0,
            };
            draw_rectangle(x, y, CELL, CELL, color);
            let glow = Color { r: 1.0, g: 0.9, b: 0.6, a: 0.4 };
            draw_rectangle(x - 3.0, y - 3.0, CELL + 6.0, CELL + 6.0, glow);
        } else {
            let color = if i == 0 { GREEN } else { LIME };
            draw_rectangle(x, y, CELL, CELL, color);
        }
    }
}

/// 绘制沙虫头部
pub fn draw_sandworm_head(x: f32, y: f32, dir: IVec2) {
    let head_color = sandworm_head_color();
    let tooth_color = Color { r: 0.95, g: 0.95, b: 0.85, a: 1.0 };
    let mouth_color = Color { r: 0.3, g: 0.15, b: 0.1, a: 1.0 };

    let expand = 2.0;
    draw_rectangle(x - expand, y - expand, CELL + expand * 2.0, CELL + expand * 2.0, head_color);

    let tooth_size = 4.0;
    let tooth_gap = 6.0;

    match (dir.x, dir.y) {
        (1, 0) => {
            let mouth_x = x + CELL - 2.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            draw_rectangle(mouth_x + 2.0, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + 2.0 + tooth_gap, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0 - tooth_gap, tooth_size, tooth_size, tooth_color);
        }
        (-1, 0) => {
            let mouth_x = x - 4.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            draw_rectangle(mouth_x, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + 2.0 + tooth_gap, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + CELL - 6.0 - tooth_gap, tooth_size, tooth_size, tooth_color);
        }
        (0, 1) => {
            let mouth_y = y + CELL - 2.0;
            draw_rectangle(x + 4.0, mouth_y, CELL - 8.0, 6.0, mouth_color);
            draw_rectangle(x + 2.0, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + 2.0 + tooth_gap, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0 - tooth_gap, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
        }
        (0, -1) => {
            let mouth_y = y - 4.0;
            draw_rectangle(x + 4.0, mouth_y, CELL - 8.0, 6.0, mouth_color);
            draw_rectangle(x + 2.0, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + 2.0 + tooth_gap, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0 - tooth_gap, mouth_y, tooth_size, tooth_size, tooth_color);
        }
        _ => {
            let mouth_x = x + CELL - 2.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            draw_rectangle(mouth_x + 2.0, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
        }
    }
}

/// 绘制沙虫身体节
pub fn draw_sandworm_segment(x: f32, y: f32, size: f32, base_color: Color, seg_x: i32, seg_y: i32, time: f32) {
    draw_rectangle(x, y, size, size, base_color);

    let spot_count = 4;
    let spot_size = size * 0.25;

    for i in 0..spot_count {
        let hash1 = ((seg_x * 7919 + seg_y * 104729 + i * 31) % 100) as f32 / 100.0;
        let hash2 = ((seg_x * 104729 + seg_y * 7919 + i * 47) % 100) as f32 / 100.0;
        let hash3 = ((seg_x * 31 + seg_y * 17 + i * 7919) % 100) as f32 / 100.0;

        let spot_x = x + hash1 * (size - spot_size);
        let spot_y = y + hash2 * (size - spot_size);

        let dark_factor = 0.3 + hash3 * 0.3;
        let spot_color = Color {
            r: base_color.r * dark_factor,
            g: base_color.g * dark_factor,
            b: base_color.b * dark_factor,
            a: 0.7 + (time * 3.0 + hash1 * 10.0).sin() * 0.2,
        };

        if hash3 < 0.6 {
            draw_rectangle(spot_x, spot_y, spot_size * (0.5 + hash3), spot_size * (0.5 + hash1), spot_color);
        }
    }
}

/// 绘制沙虫填充阶段
pub fn draw_sandworm_filling(snake: &[IVec2], _buff: &BuffState, time: f32) {
    let snake_len = snake.len() as f32;
    let sand_color = sandworm_body_color();

    for (i, seg) in snake.iter().enumerate() {
        let base_x = seg.x as f32 * CELL;
        let base_y = seg.y as f32 * CELL;
        let idx = i as f32;

        // 蠕动效果
        let wave_speed = 8.0;
        let wave_amplitude = 2.0;
        let wave_offset = (time * wave_speed + idx * 0.3).sin() * wave_amplitude;

        // 大小脉动
        let pulse_speed = 6.0;
        let pulse_amplitude = 0.1;
        let size_pulse = 1.0 + (time * pulse_speed + idx * 0.2).sin() * pulse_amplitude;
        let size = CELL * size_pulse;
        let size_offset = (CELL - size) / 2.0;

        // 颜色渐变
        let color_factor = 1.0 - (idx / snake_len.max(1.0)) * 0.15;
        let segment_color = Color {
            r: sand_color.r * color_factor,
            g: sand_color.g * color_factor,
            b: sand_color.b * color_factor,
            a: 1.0,
        };

        if i == 0 {
            let head_dir = if snake.len() > 1 {
                let next = snake[1];
                ivec2(seg.x - next.x, seg.y - next.y)
            } else {
                ivec2(1, 0)
            };
            let head_wave = (time * 10.0).sin() * 1.5;
            draw_sandworm_head(base_x + head_wave, base_y, head_dir);
        } else {
            let (offset_x, offset_y) = if i > 0 && i < snake.len() - 1 {
                let prev = snake[i - 1];
                let dx = seg.x - prev.x;
                if dx != 0 {
                    (0.0, wave_offset)
                } else {
                    (wave_offset, 0.0)
                }
            } else {
                (0.0, 0.0)
            };

            draw_sandworm_segment(
                base_x + size_offset + offset_x,
                base_y + size_offset + offset_y,
                size,
                segment_color,
                seg.x,
                seg.y,
                time
            );
        }
    }
}

/// 绘制填满后闪烁
pub fn draw_sandworm_filled_flashing(snake: &[IVec2], time: f32) {
    let flash_speed = 10.0;
    let flash = (time * flash_speed).sin() * 0.5 + 0.5;
    let sand = sandworm_body_color();
    let red = sandworm_consume_color();

    let color = Color {
        r: sand.r + (red.r - sand.r) * flash,
        g: sand.g + (red.g - sand.g) * flash,
        b: sand.b + (red.b - sand.b) * flash,
        a: 1.0,
    };

    for seg in snake {
        let x = seg.x as f32 * CELL;
        let y = seg.y as f32 * CELL;
        draw_rectangle(x, y, CELL, CELL, color);
    }
}

/// 绘制吞噬阶段（坍缩动画）
pub fn draw_sandworm_consuming(buff: &BuffState) {
    let center = buff.sandworm_collapse_center;
    let progress = buff.sandworm_collapse_progress;
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    for original_pos in buff.sandworm_collapse_positions.iter() {
        let current_x = original_pos.x + (center.x - original_pos.x) * eased_progress;
        let current_y = original_pos.y + (center.y - original_pos.y) * eased_progress;

        let size = CELL * (1.0 - eased_progress * 0.8);

        let dist_to_center = ((original_pos.x - center.x).powi(2) + (original_pos.y - center.y).powi(2)).sqrt();
        let max_dist = (GRID_W as f32 * CELL).max(GRID_H as f32 * CELL);
        let brightness = 1.0 - (dist_to_center / max_dist) * 0.5;

        let color = Color {
            r: 0.8 * brightness + 0.2,
            g: 0.1 * brightness,
            b: 0.1 * brightness,
            a: 1.0 - eased_progress * 0.3,
        };

        draw_rectangle(current_x - size / 2.0, current_y - size / 2.0, size, size, color);
    }
}

/// 绘制退出阶段
pub fn draw_sandworm_exiting(snake: &[IVec2], exit_dir: IVec2, time: f32) {
    let sand_color = sandworm_body_color();

    for (i, seg) in snake.iter().enumerate() {
        if seg.x >= 0 && seg.x < GRID_W && seg.y >= 0 && seg.y < GRID_H {
            let x = seg.x as f32 * CELL;
            let y = seg.y as f32 * CELL;
            if i == 0 {
                draw_sandworm_head(x, y, exit_dir);
            } else {
                draw_sandworm_segment(x, y, CELL, sand_color, seg.x, seg.y, time);
            }
        }
    }
}


/// 绘制沙虫模式（完整状态机渲染）
pub fn draw_sandworm_mode(snake: &[IVec2], buff: &BuffState, time: f32) {
    match buff.sandworm_phase {
        SandwormPhase::Flashing => {
            // 闪烁阶段：使用带特效的渲染
            draw_sandworm_flashing(snake, time);
        }
        SandwormPhase::Transforming => {
            // 变色阶段：使用带纹理的渲染
            draw_sandworm_transforming(snake, buff, time);
        }
        SandwormPhase::Exiting => {
            // 退出阶段：带纹理和头部
            draw_sandworm_exiting(snake, buff.sandworm_exit_dir, time);
        }
        SandwormPhase::Filling => {
            // 填充阶段：带蠕动效果和纹理
            draw_sandworm_filling(snake, buff, time);
        }
        SandwormPhase::FilledFlashing => {
            // 填满后闪烁变红：使用专用渲染
            draw_sandworm_filled_flashing(snake, time);
        }
        SandwormPhase::Consuming => {
            // 坍缩动画：使用专用渲染
            draw_sandworm_consuming(buff);
        }
        SandwormPhase::None => {}
    }
}
