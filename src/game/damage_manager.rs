//! 受伤动画管理模块
//!
//! 管理吃到陷阱果实后的受伤动画

use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use crate::types::{DamageState, DamagePhase, Particle, BloodStain};
use crate::types::{DAMAGE_FLASH_DURATION, DAMAGE_FLASH_COUNT, DAMAGE_CRUMBLE_INTERVAL, BLOOD_STAIN_LIFETIME};
use crate::constants::CELL;

/// 开始受伤动画
pub fn start_damage_animation(damage: &mut DamageState, snake: &[IVec2]) {
    damage.active = true;
    damage.phase = DamagePhase::Flashing;
    damage.flash_timer = 0.0;
    damage.flash_count = 0;
    damage.crumble_timer = 0.0;
    damage.crumble_index = 0;

    // 记录要移除的尾巴（2节）
    if snake.len() > 3 {
        damage.tail_to_remove = vec![
            snake[snake.len() - 1],
            snake[snake.len() - 2],
        ];
    } else {
        damage.tail_to_remove = vec![];
    }
}

/// 更新受伤动画
pub fn update_damage_animation(
    damage: &mut DamageState,
    snake: &mut Vec<IVec2>,
    particles: &mut Vec<Particle>,
    blood_stains: &mut Vec<BloodStain>,
    game_time: f32,
    dt: f32,
    rng: &mut ThreadRng,
) {
    match damage.phase {
        DamagePhase::Flashing => {
            damage.flash_timer += dt;
            let flash_period = DAMAGE_FLASH_DURATION / DAMAGE_FLASH_COUNT as f32;
            if damage.flash_timer >= flash_period {
                damage.flash_timer = 0.0;
                damage.flash_count += 1;

                if damage.flash_count >= DAMAGE_FLASH_COUNT {
                    damage.phase = DamagePhase::Crumbling;
                    damage.crumble_timer = 0.0;
                    damage.crumble_index = 0;
                }
            }
        }
        DamagePhase::Crumbling => {
            damage.crumble_timer += dt;

            if damage.crumble_timer >= DAMAGE_CRUMBLE_INTERVAL {
                damage.crumble_timer = 0.0;

                if damage.crumble_index < damage.tail_to_remove.len() && snake.len() > 3 {
                    let tail_pos = snake.pop().unwrap();

                    // 生成血液粒子
                    let center = vec2(
                        tail_pos.x as f32 * CELL + CELL / 2.0,
                        tail_pos.y as f32 * CELL + CELL / 2.0
                    );
                    spawn_blood_particles(particles, center, rng);

                    // 留下血迹
                    blood_stains.push(BloodStain {
                        pos: tail_pos,
                        spawn_time: game_time,
                        lifetime: BLOOD_STAIN_LIFETIME,
                        size: rng.gen_range(0.6..1.0),
                        alpha: rng.gen_range(0.5..0.8),
                    });

                    damage.crumble_index += 1;
                }

                if damage.crumble_index >= damage.tail_to_remove.len() {
                    damage.active = false;
                    damage.phase = DamagePhase::None;
                    damage.tail_to_remove.clear();
                }
            }
        }
        DamagePhase::None => {
            damage.active = false;
        }
    }
}

/// 生成血液粒子
pub fn spawn_blood_particles(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut ThreadRng) {
    let blood_colors = [
        Color { r: 0.8, g: 0.1, b: 0.1, a: 1.0 },
        Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 },
        Color { r: 0.6, g: 0.0, b: 0.0, a: 1.0 },
    ];

    for _ in 0..15 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(30.0..120.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let color = blood_colors[rng.gen_range(0..blood_colors.len())];
        let lifetime = rng.gen_range(0.4..0.8);

        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(3.0..7.0),
        });
    }
}

/// 更新血迹（移除过期的）
pub fn update_blood_stains(blood_stains: &mut Vec<BloodStain>, game_time: f32) {
    blood_stains.retain(|stain| {
        game_time - stain.spawn_time < stain.lifetime
    });
}

/// 更新粒子
pub fn update_particles(particles: &mut Vec<Particle>, dt: f32) {
    for p in particles.iter_mut() {
        p.pos += p.vel * dt;
        p.vel *= 0.95; // 阻尼
        p.lifetime -= dt;
    }
    particles.retain(|p| p.lifetime > 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_damage_animation() {
        let mut damage = DamageState::default();
        let snake = vec![ivec2(5, 5), ivec2(4, 5), ivec2(3, 5), ivec2(2, 5)];
        start_damage_animation(&mut damage, &snake);

        assert!(damage.active);
        assert_eq!(damage.phase, DamagePhase::Flashing);
        assert_eq!(damage.tail_to_remove.len(), 2);
    }

    #[test]
    fn test_particle_update() {
        let mut particles = vec![
            Particle {
                pos: vec2(0.0, 0.0),
                vel: vec2(10.0, 0.0),
                color: RED,
                lifetime: 1.0,
                max_lifetime: 1.0,
                size: 5.0,
            }
        ];

        update_particles(&mut particles, 0.5);
        assert_eq!(particles.len(), 1);
        assert!(particles[0].pos.x > 0.0);
        assert!(particles[0].lifetime < 1.0);

        update_particles(&mut particles, 1.0);
        assert_eq!(particles.len(), 0); // 粒子已过期
    }
}
