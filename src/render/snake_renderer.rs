//! 蛇渲染模块
//!
//! 提供蛇的各种视觉效果渲染

use macroquad::prelude::*;
use crate::constants::{CELL, GRID_W, GRID_H};
use crate::types::{BuffState, DamagePhase};
use crate::game::Snake;

/// 绘制蛇（带所有视觉效果）
pub fn draw_snake(
    snake: &Snake,
    buff: &BuffState,
    damage_phase: DamagePhase,
    game_time: f32,
    wrap: bool,
    blend: f32,
) {
    // 受伤闪烁效果
    let damage_flash = damage_phase == DamagePhase::Flashing && (game_time * 20.0).sin() > 0.0;

    for (i, seg) in snake.body.iter().enumerate() {
        let base_color = if i == 0 { GREEN } else { LIME };

        // 根据状态选择颜色
        let color = if damage_flash {
            Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 } // 红色（受伤）
        } else if buff.frozen {
            Color { r: 0.6, g: 0.9, b: 1.0, a: 1.0 } // 浅蓝色（冰冻）
        } else if buff.ghost_active {
            Color { r: base_color.r, g: base_color.g, b: base_color.b, a: 0.5 } // 半透明（幽灵）
        } else if buff.speed_active {
            Color { r: 0.3, g: 0.7, b: 1.0, a: 1.0 } // 蓝色（速度）
        } else if buff.slow_active {
            Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 } // 橙色（减速）
        } else if buff.dizzy_active {
            let dizzy_flash = (game_time * 15.0).sin() * 0.3 + 0.7;
            Color { r: 0.8 * dizzy_flash, g: 1.0 * dizzy_flash, b: 0.3, a: 1.0 }
        } else if buff.slime_active {
            Color { r: 0.3, g: 0.8, b: 0.3, a: 1.0 } // 绿色（粘液）
        } else {
            base_color
        };

        // 计算插值位置
        let from = snake.prev_body.get(i).unwrap_or(seg);
        let gx = from.x as f32;
        let gy = from.y as f32;
        let mut dx = seg.x as f32 - gx;
        let mut dy = seg.y as f32 - gy;

        if wrap && !buff.sandworm_active {
            let gw = GRID_W as f32;
            let gh = GRID_H as f32;
            if dx > gw * 0.5 { dx -= gw; }
            if dx < -gw * 0.5 { dx += gw; }
            if dy > gh * 0.5 { dy -= gh; }
            if dy < -gh * 0.5 { dy += gh; }
        }

        let (mut x, mut y) = if buff.frozen {
            (seg.x as f32 * CELL, seg.y as f32 * CELL)
        } else {
            ((gx + dx * blend) * CELL, (gy + dy * blend) * CELL)
        };

        // 减速抽搐效果
        if buff.slow_active {
            x += (game_time * 30.0 + i as f32 * 2.0).sin() * 2.0;
            y += (game_time * 25.0 + i as f32 * 1.5).cos() * 1.5;
        }

        // 眩晕摇晃效果
        if buff.dizzy_active {
            x += (game_time * 12.0 + i as f32 * 0.8).sin() * 3.0;
            y += (game_time * 10.0 + i as f32 * 1.2).cos() * 2.0;
        }

        // 粘液拖拽效果
        if buff.slime_active {
            let drag_factor = (i as f32 / snake.body.len() as f32) * 4.0;
            x += (game_time * 20.0 + i as f32 * 3.0).sin() * drag_factor;
            y += (game_time * 18.0 + i as f32 * 2.5).cos() * drag_factor;
        }

        draw_rectangle(x, y, CELL, CELL, color);

        // 冰冻时绘制冰晶
        if buff.frozen {
            draw_ice_crystal(x, y, game_time, seg.x, seg.y);
        }

        // 粘液滴落效果
        if buff.slime_active && i > 0 && (i + (game_time * 5.0) as usize) % 3 == 0 {
            let drip_y = (game_time * 8.0 + i as f32).sin().abs() * 5.0;
            let drip_color = Color { r: 0.2, g: 0.6, b: 0.2, a: 0.6 };
            draw_rectangle(x + CELL * 0.3, y + CELL + drip_y, 4.0, 6.0, drip_color);
        }
        
        // 炸弹在体内的效果
        if buff.bomb_state.active && i == buff.bomb_state.position {
            // 炸弹闪烁频率随位置增加
            let flash_freq = buff.bomb_state.flash_frequency(snake.body.len());
            let flash = (game_time * flash_freq * std::f32::consts::TAU).sin() * 0.5 + 0.5;
            
            // 黑色炸弹核心
            let bomb_color = Color::new(0.1, 0.1, 0.1, 0.9);
            draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, bomb_color);
            
            // 闪烁的红色警告
            let warn_color = Color::new(1.0, 0.2, 0.0, flash);
            draw_rectangle(x, y, CELL, CELL, warn_color);
            
            // 引线火花
            let spark_color = Color::new(1.0, 0.6, 0.0, flash);
            draw_rectangle(x + CELL / 2.0 - 2.0, y - 2.0, 4.0, 4.0, spark_color);
        }
        
        // 炸弹后遗症撕裂效果
        if buff.bomb_after_effect.active && i == snake.body.len().saturating_sub(1) {
            // 尾部撕裂纹理
            let bleed_flash = (game_time * 8.0).sin() * 0.3 + 0.7;
            let bleed_color = Color::new(0.8, 0.1, 0.1, bleed_flash);
            draw_rectangle(x + CELL - 4.0, y + 2.0, 4.0, CELL - 4.0, bleed_color);
            draw_rectangle(x + 2.0, y + CELL - 4.0, CELL - 4.0, 4.0, bleed_color);
        }
    }
}

/// 绘制冰晶覆盖效果
pub fn draw_ice_crystal(x: f32, y: f32, time: f32, seg_x: i32, seg_y: i32) {
    let sparkle = ((time * 8.0 + (seg_x + seg_y) as f32 * 0.5).sin() * 0.5 + 0.5).powi(2);

    // 冰晶边框
    let border_color = Color {
        r: 0.9,
        g: 0.95,
        b: 1.0,
        a: 0.4 + sparkle * 0.3,
    };
    draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);

    // 冰晶内部纹理
    let hash1 = ((seg_x * 7919 + seg_y * 104729) % 100) as f32 / 100.0;
    let hash2 = ((seg_x * 104729 + seg_y * 7919) % 100) as f32 / 100.0;
    let hash3 = ((seg_x * 31 + seg_y * 17) % 100) as f32 / 100.0;

    let crystal_color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.5 + sparkle * 0.4,
    };
    let crystal_size = 3.0 + sparkle * 2.0;

    draw_rectangle(
        x + hash1 * (CELL - crystal_size),
        y + hash2 * (CELL - crystal_size),
        crystal_size,
        crystal_size,
        crystal_color
    );
    draw_rectangle(
        x + hash2 * (CELL - crystal_size),
        y + hash3 * (CELL - crystal_size),
        crystal_size * 0.7,
        crystal_size * 0.7,
        crystal_color
    );

    // 对角线冰裂纹
    if hash1 > 0.5 {
        let crack_color = Color { r: 0.8, g: 0.9, b: 1.0, a: 0.3 };
        draw_line(x + 2.0, y + 2.0, x + CELL - 2.0, y + CELL - 2.0, 1.0, crack_color);
    }
    if hash2 > 0.5 {
        let crack_color = Color { r: 0.8, g: 0.9, b: 1.0, a: 0.3 };
        draw_line(x + CELL - 2.0, y + 2.0, x + 2.0, y + CELL - 2.0, 1.0, crack_color);
    }
}
