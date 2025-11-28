//! 特效渲染模块
//!
//! 提供粒子、血迹、传送门、残影等特效的绘制

use macroquad::prelude::*;
use crate::constants::CELL;
use crate::types::{Particle, BloodStain, Portal, Afterimage};

/// 绘制所有粒子
pub fn draw_particles(particles: &[Particle]) {
    for p in particles {
        let alpha = (p.lifetime / p.max_lifetime).clamp(0.0, 1.0);
        let size = p.size * alpha;
        let color = Color { a: alpha, ..p.color };
        draw_rectangle(p.pos.x - size / 2.0, p.pos.y - size / 2.0, size, size, color);
    }
}

/// 绘制血迹
pub fn draw_blood_stains(blood_stains: &[BloodStain], game_time: f32) {
    for stain in blood_stains {
        let elapsed = game_time - stain.spawn_time;
        let fade = 1.0 - (elapsed / stain.lifetime).clamp(0.0, 1.0);

        let x = stain.pos.x as f32 * CELL;
        let y = stain.pos.y as f32 * CELL;
        let size = CELL * stain.size;
        let offset = (CELL - size) / 2.0;

        // 主血迹
        let color = Color {
            r: 0.5,
            g: 0.0,
            b: 0.0,
            a: stain.alpha * fade,
        };
        draw_rectangle(x + offset, y + offset, size, size, color);

        // 小血滴
        let splatter_color = Color {
            r: 0.4,
            g: 0.0,
            b: 0.0,
            a: stain.alpha * fade * 0.7,
        };
        let splatter_size = size * 0.3;
        let hash1 = ((stain.pos.x * 7919 + stain.pos.y * 104729) % 100) as f32 / 100.0;
        let hash2 = ((stain.pos.x * 104729 + stain.pos.y * 7919) % 100) as f32 / 100.0;
        draw_rectangle(
            x + hash1 * (CELL - splatter_size),
            y + hash2 * (CELL - splatter_size),
            splatter_size,
            splatter_size,
            splatter_color
        );
    }
}

/// 绘制传送门
pub fn draw_portals(portals: &[Portal], game_time: f32) {
    for portal in portals {
        let elapsed = game_time - portal.spawn_time;
        let fade = if elapsed > portal.lifetime - 3.0 {
            let blink = (game_time * 8.0).sin() * 0.5 + 0.5;
            blink * (portal.lifetime - elapsed) / 3.0
        } else {
            1.0
        };

        for pos in [portal.pos_a, portal.pos_b] {
            let x = pos.x as f32 * CELL;
            let y = pos.y as f32 * CELL;

            let pulse = (game_time * 4.0).sin() * 0.15 + 0.85;

            // 外圈
            let outer_color = Color {
                r: portal.color.r,
                g: portal.color.g,
                b: portal.color.b,
                a: 0.6 * fade,
            };
            draw_rectangle(x - 2.0, y - 2.0, CELL + 4.0, CELL + 4.0, outer_color);

            // 内圈（脉动）
            let inner_size = CELL * pulse;
            let inner_offset = (CELL - inner_size) / 2.0;
            let inner_color = Color {
                r: portal.color.r * 0.5 + 0.5,
                g: portal.color.g * 0.5 + 0.5,
                b: portal.color.b * 0.5 + 0.5,
                a: 0.8 * fade,
            };
            draw_rectangle(x + inner_offset, y + inner_offset, inner_size, inner_size, inner_color);

            // 中心点
            let center_size = 6.0;
            let cx = x + CELL / 2.0 - center_size / 2.0;
            let cy = y + CELL / 2.0 - center_size / 2.0;
            draw_rectangle(cx, cy, center_size, center_size, Color { a: fade, ..WHITE });
        }

        // 连接线
        let ax = portal.pos_a.x as f32 * CELL + CELL / 2.0;
        let ay = portal.pos_a.y as f32 * CELL + CELL / 2.0;
        let bx = portal.pos_b.x as f32 * CELL + CELL / 2.0;
        let by = portal.pos_b.y as f32 * CELL + CELL / 2.0;
        let line_color = Color { a: 0.3 * fade, ..portal.color };
        draw_line(ax, ay, bx, by, 1.0, line_color);
    }
}

/// 绘制残影
pub fn draw_afterimages(afterimages: &[Afterimage], game_time: f32) {
    const AFTERIMAGE_LIFETIME: f32 = 0.3;

    for afterimage in afterimages {
        let elapsed = game_time - afterimage.spawn_time;
        let fade = 1.0 - (elapsed / AFTERIMAGE_LIFETIME);
        let alpha = afterimage.alpha * fade;

        let color = Color {
            r: 0.2,
            g: 0.5,
            b: 1.0,
            a: alpha * 0.5,
        };

        for seg in &afterimage.positions {
            let x = seg.x as f32 * CELL;
            let y = seg.y as f32 * CELL;
            draw_rectangle(x, y, CELL, CELL, color);
        }
    }
}

/// 绘制护盾效果
pub fn draw_shield_effect(snake_body: &[IVec2], time: f32) {
    let pulse = (time * 4.0).sin() * 0.2 + 0.8;
    let gold = Color { r: 1.0, g: 0.84, b: 0.0, a: 0.3 * pulse };

    for seg in snake_body {
        let x = seg.x as f32 * CELL - 2.0;
        let y = seg.y as f32 * CELL - 2.0;
        let size = CELL + 4.0;
        draw_rectangle(x, y, size, size, gold);
    }
}

/// 绘制幽灵效果
pub fn draw_ghost_effect(snake_body: &[IVec2], time: f32) {
    let pulse = (time * 6.0).sin() * 0.3 + 0.7;
    let ghost_color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.2 * pulse };

    for seg in snake_body {
        let x = seg.x as f32 * CELL - 3.0;
        let y = seg.y as f32 * CELL - 3.0;
        let size = CELL + 6.0;
        draw_rectangle(x, y, size, size, ghost_color);
    }
}

/// 生成冰冻粒子
pub fn spawn_freeze_particles(particles: &mut Vec<Particle>, snake_body: &[IVec2], rng: &mut impl ::rand::Rng) {
    let ice_colors = [
        Color { r: 0.7, g: 0.9, b: 1.0, a: 1.0 },
        Color { r: 0.5, g: 0.8, b: 1.0, a: 1.0 },
        Color { r: 0.9, g: 0.95, b: 1.0, a: 1.0 },
        Color { r: 0.6, g: 0.85, b: 0.95, a: 1.0 },
    ];

    for seg in snake_body {
        let center = vec2(
            seg.x as f32 * CELL + CELL / 2.0,
            seg.y as f32 * CELL + CELL / 2.0
        );

        for _ in 0..rng.gen_range(5..8) {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(20.0..80.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed - 30.0);
            let color = ice_colors[rng.gen_range(0..ice_colors.len())];
            let lifetime = rng.gen_range(0.5..1.2);

            particles.push(Particle {
                pos: center,
                vel,
                color,
                lifetime,
                max_lifetime: lifetime,
                size: rng.gen_range(2.0..5.0),
            });
        }
    }
}

/// 生成幸运方块粒子
pub fn spawn_lucky_particles(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut impl ::rand::Rng) {
    let colors = [
        Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 },
        Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 },
        Color { r: 1.0, g: 1.0, b: 0.2, a: 1.0 },
        Color { r: 0.2, g: 1.0, b: 0.2, a: 1.0 },
        Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 },
        Color { r: 0.8, g: 0.2, b: 1.0, a: 1.0 },
    ];

    for _ in 0..25 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(80.0..200.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let color = colors[rng.gen_range(0..colors.len())];
        let lifetime = rng.gen_range(0.5..1.0);

        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(3.0..8.0),
        });
    }
}
