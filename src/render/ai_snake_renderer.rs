//! AI 蛇渲染模块

use macroquad::prelude::*;
use crate::constants::CELL;
use crate::game::{AISnake, DroppedFood};
use crate::types::SandwormPhase;

/// 绘制所有 AI 蛇
pub fn draw_ai_snakes(snakes: &[AISnake], game_time: f32, _blend: f32) {
    for snake in snakes {
        draw_ai_snake(snake, game_time);
    }
}

/// 绘制单条 AI 蛇
fn draw_ai_snake(snake: &AISnake, game_time: f32) {
    let body = &snake.body;
    let prev_body = &snake.prev_body;
    let color = snake.color;
    
    // 计算 AI 蛇自己的插值比例
    let tick = snake.get_tick();
    let ai_blend = if tick > 0.0 {
        (snake.move_accumulator / tick).clamp(0.0, 1.0)
    } else {
        0.0
    };
    
    // 冰冻效果
    let is_frozen = snake.buff_state.frozen;
    let frozen_tint = if is_frozen {
        Color::new(0.7, 0.9, 1.0, 0.8)
    } else {
        WHITE
    };
    
    // 减速效果（紫色光晕）
    let is_slowed = snake.buff_state.slow_active;
    
    for (i, &pos) in body.iter().enumerate() {
        // 插值位置 - 使用 AI 蛇自己的 blend
        let prev_pos = prev_body.get(i).copied().unwrap_or(pos);
        
        // 检测是否发生穿墙（位置差距超过2格）
        let dx = (pos.x - prev_pos.x).abs();
        let dy = (pos.y - prev_pos.y).abs();
        let is_wrap = dx > 2 || dy > 2;
        
        // 穿墙时不插值，直接使用当前位置
        let (lerp_x, lerp_y) = if is_wrap {
            (pos.x as f32, pos.y as f32)
        } else {
            (
                prev_pos.x as f32 + (pos.x - prev_pos.x) as f32 * ai_blend,
                prev_pos.y as f32 + (pos.y - prev_pos.y) as f32 * ai_blend,
            )
        };
        
        let x = lerp_x * CELL;
        let y = lerp_y * CELL;
        
        // 身体颜色渐变（头部最亮）
        let brightness = 1.0 - (i as f32 / body.len() as f32) * 0.4;
        let segment_color = Color::new(
            color.r * brightness * frozen_tint.r,
            color.g * brightness * frozen_tint.g,
            color.b * brightness * frozen_tint.b,
            color.a * frozen_tint.a,
        );
        
        // 减速光晕
        if is_slowed {
            draw_rectangle(
                x - 2.0,
                y - 2.0,
                CELL + 4.0,
                CELL + 4.0,
                Color::new(0.5, 0.0, 0.5, 0.3 + (game_time * 3.0).sin() * 0.1),
            );
        }
        
        // 绘制身体段
        if i == 0 {
            // 蛇头 - 稍大一点
            draw_rectangle(x - 1.0, y - 1.0, CELL + 2.0, CELL + 2.0, segment_color);
            
            // 眼睛
            let eye_color = if is_frozen { SKYBLUE } else { WHITE };
            let eye_size = 4.0;
            let eye_offset = 5.0;
            
            // 根据方向绘制眼睛
            let (eye1, eye2) = match (snake.dir.x, snake.dir.y) {
                (1, 0) => (  // 向右
                    (x + CELL - eye_offset, y + eye_offset),
                    (x + CELL - eye_offset, y + CELL - eye_offset - eye_size),
                ),
                (-1, 0) => ( // 向左
                    (x + eye_offset - eye_size, y + eye_offset),
                    (x + eye_offset - eye_size, y + CELL - eye_offset - eye_size),
                ),
                (0, 1) => (  // 向下
                    (x + eye_offset, y + CELL - eye_offset),
                    (x + CELL - eye_offset - eye_size, y + CELL - eye_offset),
                ),
                _ => (       // 向上
                    (x + eye_offset, y + eye_offset - eye_size),
                    (x + CELL - eye_offset - eye_size, y + eye_offset - eye_size),
                ),
            };
            
            draw_rectangle(eye1.0, eye1.1, eye_size, eye_size, eye_color);
            draw_rectangle(eye2.0, eye2.1, eye_size, eye_size, eye_color);
            
            // 瞳孔
            draw_rectangle(eye1.0 + 1.0, eye1.1 + 1.0, 2.0, 2.0, BLACK);
            draw_rectangle(eye2.0 + 1.0, eye2.1 + 1.0, 2.0, 2.0, BLACK);
        } else {
            // 身体段
            draw_rectangle(x + 1.0, y + 1.0, CELL - 2.0, CELL - 2.0, segment_color);
        }
        
        // 眩晕效果（头顶星星）
        if i == 0 && snake.buff_state.dizzy_active {
            let star_offset = (game_time * 5.0).sin() * 3.0;
            draw_text("★", x + 2.0 + star_offset, y - 2.0, 12.0, YELLOW);
            draw_text("★", x + CELL - 8.0 - star_offset, y - 2.0, 12.0, YELLOW);
        }
    }
}

/// 绘制掉落的食物
pub fn draw_dropped_foods(foods: &[DroppedFood], game_time: f32) {
    for food in foods {
        let x = food.pos.x as f32 * CELL;
        let y = food.pos.y as f32 * CELL;
        
        // 计算剩余时间，快消失时闪烁
        let remaining = food.lifetime - (game_time - food.spawn_time);
        let alpha = if remaining < 3.0 {
            0.5 + (game_time * 8.0).sin() * 0.3
        } else {
            1.0
        };
        
        // 绘制食物（金色小方块）
        let color = Color::new(1.0, 0.85, 0.0, alpha);
        draw_rectangle(x + 4.0, y + 4.0, CELL - 8.0, CELL - 8.0, color);
        
        // 高光
        draw_rectangle(x + 5.0, y + 5.0, 3.0, 3.0, Color::new(1.0, 1.0, 1.0, alpha * 0.5));
    }
}

/// 绘制 AI 蛇被沙虫吞噬的效果
pub fn draw_ai_snake_sandworm_effect(snake: &AISnake, phase: SandwormPhase, game_time: f32) {
    if phase == SandwormPhase::None {
        return;
    }
    
    // 沙虫模式下 AI 蛇变暗并抖动
    for (i, &pos) in snake.body.iter().enumerate() {
        let shake = (game_time * 20.0 + i as f32).sin() * 2.0;
        let x = pos.x as f32 * CELL + shake;
        let y = pos.y as f32 * CELL;
        
        let alpha = 0.3 + (game_time * 5.0).sin() * 0.1;
        let color = Color::new(snake.color.r * 0.5, snake.color.g * 0.5, snake.color.b * 0.5, alpha);
        
        draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, color);
    }
}
