//! 炸弹管理器
//!
//! 封装炸弹爆炸和后遗症的所有逻辑

use macroquad::prelude::*;
use ::rand::Rng;
use crate::constants::CELL;
use crate::types::{BuffState, Particle};

/// 炸弹更新结果
pub struct BombUpdateResult {
    /// 是否导致游戏结束
    pub game_over: bool,
    /// 生成的粒子
    pub particles: Vec<Particle>,
    /// 需要截断蛇身到的位置（None表示不截断）
    pub truncate_to: Option<usize>,
}

impl Default for BombUpdateResult {
    fn default() -> Self {
        BombUpdateResult {
            game_over: false,
            particles: Vec::new(),
            truncate_to: None,
        }
    }
}

/// 炸弹管理器
pub struct BombManager;

impl BombManager {
    /// 更新炸弹状态
    /// 
    /// 处理炸弹在体内移动、爆炸、后遗症掉血
    pub fn update<R: Rng>(
        buff_state: &mut BuffState,
        snake_body: &[IVec2],
        dt: f32,
        rng: &mut R,
    ) -> BombUpdateResult {
        let mut result = BombUpdateResult::default();
        let snake_len = snake_body.len();

        // 更新炸弹移动
        if let Some(explode_pos) = buff_state.update_bomb(dt, snake_len) {
            if explode_pos < snake_len {
                // 生成爆炸粒子效果
                let explode_cell = snake_body[explode_pos];
                let center = vec2(
                    explode_cell.x as f32 * CELL + CELL / 2.0,
                    explode_cell.y as f32 * CELL + CELL / 2.0,
                );

                // 橙红色爆炸粒子
                for _ in 0..30 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(80.0..200.0);
                    let lifetime = rng.gen_range(0.3..0.7);
                    let color = if rng.gen_bool(0.5) {
                        Color::new(1.0, 0.4, 0.0, 1.0) // 橙色
                    } else {
                        Color::new(1.0, 0.2, 0.0, 1.0) // 红色
                    };
                    result.particles.push(Particle {
                        pos: center,
                        vel: vec2(angle.cos() * speed, angle.sin() * speed),
                        color,
                        lifetime,
                        max_lifetime: lifetime,
                        size: rng.gen_range(4.0..8.0),
                    });
                }

                // 标记截断位置
                result.truncate_to = Some(explode_pos);

                // 激活后遗症
                buff_state.bomb_after_effect.activate(explode_pos);

                // 检查蛇是否太短导致死亡
                if explode_pos < 3 {
                    result.game_over = true;
                }
            }
        }

        result
    }

    /// 更新炸弹后遗症（掉血）
    /// 
    /// 返回是否需要移除尾部，以及是否游戏结束
    pub fn update_after_effect(
        buff_state: &mut BuffState,
        snake_len: usize,
        dt: f32,
    ) -> (bool, bool) {
        if !buff_state.bomb_after_effect.active {
            return (false, false);
        }
        
        // 更新后遗症计时器
        buff_state.bomb_after_effect.timer -= dt;
        if buff_state.bomb_after_effect.timer <= 0.0 {
            buff_state.bomb_after_effect.clear();
            return (false, false);
        }

        // 每3秒掉一格
        buff_state.bomb_after_effect.bleed_timer += dt;
        if buff_state.bomb_after_effect.bleed_timer >= 3.0 {
            buff_state.bomb_after_effect.bleed_timer -= 3.0;
            
            if snake_len > 3 {
                return (true, false); // 需要掉血
            } else {
                return (false, true); // 游戏结束
            }
        }

        (false, false)
    }
}
