//! 果实消费处理模块
//!
//! 处理 FruitBehavior::on_consume 的结果

use macroquad::prelude::*;
use ::rand::rngs::ThreadRng;
use crate::types::{Fruit, BuffState, DamageState, Particle, ComboState, GameState};
use crate::fruits::{FruitRegistry, FruitContext, ConsumeResult};
use crate::game::Snake;

/// 处理果实消费结果
///
/// # 返回
/// 如果游戏应该结束返回 true
pub fn handle_consume_result(
    result: ConsumeResult,
    score: &mut u32,
    combo: &mut ComboState,
    game_state: &mut GameState,
) -> bool {
    match result {
        ConsumeResult::Continue => false,
        ConsumeResult::GameOver => {
            *game_state = GameState::GameOver;
            true
        }
        ConsumeResult::ResetCombo => {
            combo.count = 0;
            false
        }
        ConsumeResult::AddScore(points) => {
            *score += points;
            false
        }
        ConsumeResult::Multiple(results) => {
            let mut game_over = false;
            for r in results {
                if handle_consume_result(r, score, combo, game_state) {
                    game_over = true;
                }
            }
            game_over
        }
    }
}

/// 消费果实并处理效果
///
/// # 返回
/// (是否消费了果实, 是否游戏结束)
pub fn consume_fruit(
    fruit_idx: usize,
    fruits: &mut Vec<Fruit>,
    registry: &FruitRegistry,
    snake: &mut Snake,
    buff_state: &mut BuffState,
    damage_state: &mut DamageState,
    particles: &mut Vec<Particle>,
    score: &mut u32,
    combo_state: &mut ComboState,
    game_state: &mut GameState,
    rng: &mut ThreadRng,
    game_time: f32,
) -> (bool, bool) {
    if fruit_idx >= fruits.len() {
        return (false, false);
    }

    let fruit = fruits.remove(fruit_idx);
    let fruit_pos = fruit.pos;

    // 获取果实行为
    let behavior = match registry.get(fruit.type_id) {
        Some(b) => b,
        None => return (true, false),
    };

    // 创建上下文
    let mut ctx = FruitContext {
        snake: &mut snake.body,
        dir: &mut snake.dir,
        buff_state,
        particles,
        damage_state,
        score,
        combo_state,
        rng,
        game_time,
        fruit_pos,
    };

    // 调用 on_consume
    let result = behavior.on_consume(&mut ctx);

    // 处理结果
    let game_over = handle_consume_result(result, score, combo_state, game_state);

    (true, game_over)
}

/// 更新 Combo 状态
pub fn update_combo(combo: &mut ComboState, game_time: f32) {
    const COMBO_WINDOW: f32 = 1.5;

    let time_since_last = game_time - combo.last_eat_time;

    if time_since_last <= COMBO_WINDOW {
        combo.count += 1;
    } else {
        combo.count = 1;
    }

    combo.last_eat_time = game_time;
    combo.display_timer = 1.5;
}

use ::rand::Rng;
use crate::constants::CELL;
use crate::fruits::FruitCategory;
use crate::types::SandwormPhase;
use crate::game::start_damage_animation;
use crate::render::spawn_lucky_particles;
use crate::audio::SoundManager;

/// 处理果实效果（完整版）
///
/// 根据果实类型执行相应的游戏效果：
/// - Normal: 加分、增长
/// - Trap: 负面效果（可被护盾免疫）
/// - Power: 激活增益Buff
/// - Special: 随机效果
///
/// # 参数
/// - `fruit`: 被吃掉的果实
/// - `registry`: 果实注册表
/// - `snake`: 蛇实例
/// - `buff_state`: Buff状态
/// - `damage_state`: 受伤状态
/// - `particles`: 粒子列表
/// - `score`: 当前分数
/// - `combo_state`: Combo状态
/// - `game_state`: 游戏状态
/// - `rng`: 随机数生成器
/// - `sounds`: 音效管理器
pub fn handle_fruit_effect(
    fruit: &Fruit,
    registry: &FruitRegistry,
    snake: &mut crate::game::Snake,
    buff_state: &mut BuffState,
    damage_state: &mut DamageState,
    particles: &mut Vec<Particle>,
    score: &mut u32,
    combo_state: &mut ComboState,
    game_state: &mut GameState,
    rng: &mut impl Rng,
    _game_time: f32,
    sounds: &SoundManager,
) {
    let config = match registry.get_config(fruit.type_id) {
        Some(c) => c,
        None => return,
    };

    match config.category {
        FruitCategory::Normal => {
            // 普通果实：加分 + 增长
            let combo_bonus = combo_state.count.min(5);
            *score += 1 + combo_bonus;
            snake.grow();
            sounds.play_eat();
        }
        FruitCategory::Trap => {
            // 陷阱果实：负面效果
            sounds.play_trap();
            if !buff_state.has_immunity() {
                match fruit.type_id {
                    "trap" => {
                        // 受伤：蛇太短则游戏结束
                        if snake.len() <= 3 {
                            *game_state = GameState::GameOver;
                        } else {
                            start_damage_animation(damage_state, &snake.body);
                        }
                    }
                    "freeze" => buff_state.activate_freeze(),
                    "slow" => buff_state.activate_slow(),
                    "dizzy" => buff_state.activate_dizzy(),
                    "slime" => buff_state.activate_slime(),
                    _ => {}
                }
            }
            combo_state.count = 0;
        }
        FruitCategory::Power => {
            // 功能果实：激活增益
            sounds.play_power();
            match fruit.type_id {
                "shield" => buff_state.activate_shield(),
                "speed" => buff_state.activate_speed(),
                "ghost" => buff_state.activate_ghost(),
                "reverse" => snake.reverse(),
                "sandworm" => {
                    // 激活沙虫模式
                    buff_state.sandworm_active = true;
                    buff_state.sandworm_phase = SandwormPhase::Flashing;
                    buff_state.sandworm_phase_timer = 0.0;
                    buff_state.sandworm_transform_index = 0;
                    buff_state.sandworm_tick = 0.0;
                    // 保存原始状态，坍缩后恢复
                    buff_state.sandworm_original_snake = snake.body.clone();
                    buff_state.sandworm_original_dir = snake.dir;
                }
                _ => {}
            }
        }
        FruitCategory::Special => {
            // 特殊果实：随机效果
            sounds.play_power();

            // 生成彩色粒子
            let center = vec2(
                fruit.pos.x as f32 * CELL + CELL / 2.0,
                fruit.pos.y as f32 * CELL + CELL / 2.0,
            );
            spawn_lucky_particles(particles, center, rng);

            if rng.gen_bool(0.5) {
                // 50% 概率获得增益
                match rng.gen_range(0..5) {
                    0 => buff_state.activate_shield(),
                    1 => buff_state.activate_speed(),
                    2 => buff_state.activate_ghost(),
                    3 => *score += 10,
                    _ => {
                        for _ in 0..3 {
                            snake.grow();
                        }
                    }
                }
            } else if !buff_state.has_immunity() {
                // 50% 概率获得负面效果（可被免疫）
                match rng.gen_range(0..5) {
                    0 => buff_state.activate_freeze(),
                    1 => buff_state.activate_slow(),
                    2 => buff_state.activate_dizzy(),
                    3 => buff_state.activate_slime(),
                    _ => {
                        if snake.len() > 3 {
                            start_damage_animation(damage_state, &snake.body);
                        }
                    }
                }
            }
            combo_state.count = 0;
        }
    }
}
