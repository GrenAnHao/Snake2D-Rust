//! 沙虫模式管理模块
//!
//! 处理沙虫变身的完整状态机逻辑

use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use crate::constants::{
    CELL, GRID_W, GRID_H,
    SANDWORM_TICK_INTERVAL, SANDWORM_FLASH_DURATION,
    SANDWORM_TRANSFORM_INTERVAL, SANDWORM_FILLED_FLASH_DURATION,
};
use crate::types::{BuffState, SandwormPhase, Fruit, Particle};
use crate::game::{Snake, spawn_food};

/// 沙虫模式更新结果
pub struct SandwormUpdateResult {
    /// 是否需要重新生成食物
    pub need_respawn_food: bool,
    /// 奖励分数
    pub bonus_score: u32,
}

/// 更新沙虫模式状态机
///
/// 沙虫模式阶段：
/// 1. Flashing: 原地闪烁
/// 2. Transforming: 从头到尾变成沙色
/// 3. Exiting: 移出边界
/// 4. Filling: 填满整个屏幕（吞噬所有果实）
/// 5. FilledFlashing: 填满后闪烁变红
/// 6. Consuming: 坍缩动画
/// 7. 恢复原状态继续游戏
pub fn update_sandworm_mode(
    snake: &mut Snake,
    buff: &mut BuffState,
    fruits: &mut Vec<Fruit>,
    food: &mut IVec2,
    particles: &mut Vec<Particle>,
    dt: f32,
    rng: &mut ThreadRng,
) -> SandwormUpdateResult {
    let mut result = SandwormUpdateResult {
        need_respawn_food: false,
        bonus_score: 0,
    };

    if buff.sandworm_phase == SandwormPhase::None {
        return result;
    }

    buff.sandworm_phase_timer += dt;

    match buff.sandworm_phase {
        SandwormPhase::Flashing => {
            // 闪烁阶段
            if buff.sandworm_phase_timer >= SANDWORM_FLASH_DURATION {
                buff.sandworm_phase = SandwormPhase::Transforming;
                buff.sandworm_phase_timer = 0.0;
                buff.sandworm_transform_index = 0;
            }
        }
        SandwormPhase::Transforming => {
            // 逐节变色
            if buff.sandworm_phase_timer >= SANDWORM_TRANSFORM_INTERVAL {
                buff.sandworm_phase_timer = 0.0;
                buff.sandworm_transform_index += 1;

                if buff.sandworm_transform_index >= snake.len() {
                    // 变色完成，开始退出
                    buff.sandworm_phase = SandwormPhase::Exiting;
                    buff.sandworm_phase_timer = 0.0;

                    // 选择退出方向（向最近的边界）
                    let head = snake.head();
                    let to_left = head.x;
                    let to_right = GRID_W - 1 - head.x;
                    let to_top = head.y;
                    let to_bottom = GRID_H - 1 - head.y;

                    let min_dist = to_left.min(to_right).min(to_top).min(to_bottom);
                    buff.sandworm_exit_dir = if min_dist == to_left {
                        ivec2(-1, 0)
                    } else if min_dist == to_right {
                        ivec2(1, 0)
                    } else if min_dist == to_top {
                        ivec2(0, -1)
                    } else {
                        ivec2(0, 1)
                    };
                }
            }
        }
        SandwormPhase::Exiting => {
            // 快速移出边界
            buff.sandworm_tick += dt;
            if buff.sandworm_tick >= SANDWORM_TICK_INTERVAL {
                buff.sandworm_tick = 0.0;

                // 移动蛇
                let head = snake.head();
                let new_head = ivec2(
                    head.x + buff.sandworm_exit_dir.x,
                    head.y + buff.sandworm_exit_dir.y,
                );
                snake.body.insert(0, new_head);
                snake.body.pop();

                // 检查是否完全离开屏幕
                let all_outside = snake.body.iter().all(|seg| {
                    seg.x < 0 || seg.x >= GRID_W || seg.y < 0 || seg.y >= GRID_H
                });

                if all_outside {
                    // 开始填充阶段
                    buff.sandworm_phase = SandwormPhase::Filling;
                    buff.sandworm_phase_timer = 0.0;

                    // 从对面边界进入
                    let entry_dir = ivec2(-buff.sandworm_exit_dir.x, -buff.sandworm_exit_dir.y);
                    let entry_pos = if entry_dir.x == 1 {
                        ivec2(0, GRID_H / 2)
                    } else if entry_dir.x == -1 {
                        ivec2(GRID_W - 1, GRID_H / 2)
                    } else if entry_dir.y == 1 {
                        ivec2(GRID_W / 2, 0)
                    } else {
                        ivec2(GRID_W / 2, GRID_H - 1)
                    };

                    // 生成填充路径（蛇形扫描）
                    buff.sandworm_path = generate_fill_path(entry_pos, entry_dir);
                    buff.sandworm_index = 0;

                    // 重置蛇身为起点
                    snake.body = vec![entry_pos];
                    snake.dir = entry_dir;
                }
            }
        }
        SandwormPhase::Filling => {
            // 填充整个屏幕
            buff.sandworm_tick += dt;
            if buff.sandworm_tick >= SANDWORM_TICK_INTERVAL {
                buff.sandworm_tick = 0.0;

                if buff.sandworm_index < buff.sandworm_path.len() {
                    let next_pos = buff.sandworm_path[buff.sandworm_index];
                    snake.body.insert(0, next_pos);
                    buff.sandworm_index += 1;

                    // 沙虫期间吞噬所有果实
                    fruits.retain(|f| {
                        if f.pos == next_pos {
                            result.bonus_score += 5; // 吞噬果实加分
                            false
                        } else {
                            true
                        }
                    });

                    // 吞噬食物
                    if *food == next_pos {
                        result.bonus_score += 1;
                        *food = ivec2(-100, -100); // 移到屏幕外
                    }
                } else {
                    // 填充完成，开始闪烁
                    buff.sandworm_phase = SandwormPhase::FilledFlashing;
                    buff.sandworm_phase_timer = 0.0;
                }
            }
        }
        SandwormPhase::FilledFlashing => {
            // 填满后闪烁
            if buff.sandworm_phase_timer >= SANDWORM_FILLED_FLASH_DURATION {
                buff.sandworm_phase = SandwormPhase::Consuming;
                buff.sandworm_phase_timer = 0.0;
                buff.sandworm_collapse_progress = 0.0;

                // 计算坍缩中心
                let cx = GRID_W as f32 * CELL / 2.0;
                let cy = GRID_H as f32 * CELL / 2.0;
                buff.sandworm_collapse_center = vec2(cx, cy);

                // 保存所有位置用于坍缩动画
                buff.sandworm_collapse_positions = snake
                    .body
                    .iter()
                    .map(|seg| {
                        vec2(
                            seg.x as f32 * CELL + CELL / 2.0,
                            seg.y as f32 * CELL + CELL / 2.0,
                        )
                    })
                    .collect();
            }
        }
        SandwormPhase::Consuming => {
            // 坍缩动画
            buff.sandworm_collapse_progress += dt * 0.8; // 坍缩速度

            if buff.sandworm_collapse_progress >= 1.0 {
                // 坍缩完成，恢复原状态继续游戏
                snake.body = buff.sandworm_original_snake.clone();
                snake.dir = buff.sandworm_original_dir;
                snake.prev_body = snake.body.clone();

                // 生成爆炸粒子
                let center = buff.sandworm_collapse_center;
                for _ in 0..50 {
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let speed = rng.gen_range(100.0..300.0);
                    particles.push(Particle {
                        pos: center,
                        vel: vec2(angle.cos() * speed, angle.sin() * speed),
                        color: Color {
                            r: 0.76,
                            g: 0.60,
                            b: 0.42,
                            a: 1.0,
                        },
                        lifetime: 1.0,
                        max_lifetime: 1.0,
                        size: rng.gen_range(5.0..15.0),
                    });
                }

                // 重置沙虫状态
                buff.sandworm_active = false;
                buff.sandworm_phase = SandwormPhase::None;
                buff.sandworm_path.clear();
                buff.sandworm_original_snake.clear();
                buff.sandworm_collapse_positions.clear();

                // 重新生成食物
                *food = spawn_food(&snake.body, rng);

                // 奖励分数
                result.bonus_score += 100;
                result.need_respawn_food = true;
            }
        }
        SandwormPhase::None => {}
    }

    result
}

/// 生成蛇形扫描填充路径
pub fn generate_fill_path(start: IVec2, initial_dir: IVec2) -> Vec<IVec2> {
    let mut path = Vec::new();
    let mut pos = start;
    let mut dir = initial_dir;
    let mut visited = vec![vec![false; GRID_H as usize]; GRID_W as usize];

    // 蛇形扫描整个屏幕
    let total_cells = (GRID_W * GRID_H) as usize;

    for _ in 0..total_cells {
        // 标记当前位置
        if pos.x >= 0 && pos.x < GRID_W && pos.y >= 0 && pos.y < GRID_H {
            if !visited[pos.x as usize][pos.y as usize] {
                visited[pos.x as usize][pos.y as usize] = true;
                path.push(pos);
            }
        }

        // 尝试继续前进
        let next = ivec2(pos.x + dir.x, pos.y + dir.y);
        if next.x >= 0
            && next.x < GRID_W
            && next.y >= 0
            && next.y < GRID_H
            && !visited[next.x as usize][next.y as usize]
        {
            pos = next;
        } else {
            // 需要转向 - 尝试右转
            let right_dir = ivec2(-dir.y, dir.x);
            let right_next = ivec2(pos.x + right_dir.x, pos.y + right_dir.y);
            if right_next.x >= 0
                && right_next.x < GRID_W
                && right_next.y >= 0
                && right_next.y < GRID_H
                && !visited[right_next.x as usize][right_next.y as usize]
            {
                dir = right_dir;
                pos = right_next;
            } else {
                // 尝试左转
                let left_dir = ivec2(dir.y, -dir.x);
                let left_next = ivec2(pos.x + left_dir.x, pos.y + left_dir.y);
                if left_next.x >= 0
                    && left_next.x < GRID_W
                    && left_next.y >= 0
                    && left_next.y < GRID_H
                    && !visited[left_next.x as usize][left_next.y as usize]
                {
                    dir = left_dir;
                    pos = left_next;
                } else {
                    // 尝试后退
                    let back_dir = ivec2(-dir.x, -dir.y);
                    let back_next = ivec2(pos.x + back_dir.x, pos.y + back_dir.y);
                    if back_next.x >= 0
                        && back_next.x < GRID_W
                        && back_next.y >= 0
                        && back_next.y < GRID_H
                        && !visited[back_next.x as usize][back_next.y as usize]
                    {
                        dir = back_dir;
                        pos = back_next;
                    } else {
                        // 找任意未访问的相邻格子
                        let neighbors = [
                            ivec2(pos.x + 1, pos.y),
                            ivec2(pos.x - 1, pos.y),
                            ivec2(pos.x, pos.y + 1),
                            ivec2(pos.x, pos.y - 1),
                        ];
                        let mut found = false;
                        for n in neighbors {
                            if n.x >= 0
                                && n.x < GRID_W
                                && n.y >= 0
                                && n.y < GRID_H
                                && !visited[n.x as usize][n.y as usize]
                            {
                                dir = ivec2(n.x - pos.x, n.y - pos.y);
                                pos = n;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            break; // 无法继续
                        }
                    }
                }
            }
        }
    }

    path
}
