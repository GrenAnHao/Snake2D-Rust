//! AI 蛇管理器模块
//!
//! 管理所有 AI 蛇的生成、更新、碰撞检测和死亡处理。

use macroquad::prelude::*;
use ::rand::Rng;
use crate::types::{Fruit, Particle};
use crate::fruits::{FruitRegistry, FruitCategory};
use super::ai_snake::{AISnake, AIMoveResult};

/// 掉落的食物（AI蛇死亡后产生）
#[derive(Clone)]
pub struct DroppedFood {
    pub pos: IVec2,
    pub spawn_time: f32,
    pub lifetime: f32,
}

/// AI 蛇管理器
pub struct AIManager {
    /// 所有 AI 蛇
    pub snakes: Vec<AISnake>,
    /// 最大 AI 蛇数量
    pub max_snakes: usize,
    /// 下一个 AI 蛇 ID
    next_id: u32,
    /// 掉落的食物列表
    pub dropped_foods: Vec<DroppedFood>,
}

impl Default for AIManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AIManager {
    pub fn new() -> Self {
        AIManager {
            snakes: Vec::new(),
            max_snakes: 3,
            next_id: 1,
            dropped_foods: Vec::new(),
        }
    }
    
    /// 重置管理器
    pub fn reset(&mut self) {
        self.snakes.clear();
        self.dropped_foods.clear();
        self.next_id = 1;
    }
    
    /// 生成新的 AI 蛇
    pub fn spawn_snake<R: Rng>(
        &mut self,
        player_body: &[IVec2],
        rng: &mut R,
    ) -> bool {
        if self.snakes.len() >= self.max_snakes {
            return false;
        }
        
        // 收集所有已占用位置
        let mut occupied: Vec<IVec2> = player_body.to_vec();
        for snake in &self.snakes {
            occupied.extend(&snake.body);
        }
        
        if let Some(snake) = AISnake::new(self.next_id, &occupied, rng) {
            self.next_id += 1;
            self.snakes.push(snake);
            true
        } else {
            false
        }
    }
    
    /// 更新所有 AI 蛇的决策
    pub fn update_thinking<R: Rng>(
        &mut self,
        food: IVec2,
        fruits: &[Fruit],
        player_body: &[IVec2],
        wrap: bool,
        dt: f32,
        rng: &mut R,
    ) {
        // 收集其他 AI 蛇的身体位置
        let bodies: Vec<Vec<IVec2>> = self.snakes.iter()
            .map(|s| s.body.clone())
            .collect();
        
        for (i, snake) in self.snakes.iter_mut().enumerate() {
            snake.think_timer += dt;
            
            // 每 0.2 秒做一次决策
            if snake.think_timer >= 0.2 {
                snake.think_timer = 0.0;
                
                // 排除自己的身体
                let other_bodies: Vec<Vec<IVec2>> = bodies.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, b)| b.clone())
                    .collect();
                
                snake.think(food, fruits, player_body, &other_bodies, wrap, rng);
            }
        }
    }
    
    /// 移动所有 AI 蛇并处理碰撞
    pub fn update_movement<R: Rng>(
        &mut self,
        food: &mut IVec2,
        fruits: &mut Vec<Fruit>,
        player_body: &[IVec2],
        player_has_immunity: bool,
        particles: &mut Vec<Particle>,
        registry: &FruitRegistry,
        wrap: bool,
        game_time: f32,
        dt: f32,
        rng: &mut R,
    ) -> AIUpdateResult {
        let mut result = AIUpdateResult::default();
        let mut dead_indices: Vec<usize> = Vec::new();
        let mut need_respawn_food = false;
        
        // 收集所有 AI 蛇的身体位置（用于互相碰撞检测）
        let bodies: Vec<Vec<IVec2>> = self.snakes.iter()
            .map(|s| s.body.clone())
            .collect();
        
        for (i, snake) in self.snakes.iter_mut().enumerate() {
            // 更新 Buff 状态
            snake.buff_state.update(dt);
            
            // 累积时间
            snake.move_accumulator += dt;
            let tick = snake.get_tick();
            
            // 检查是否该移动了
            if snake.move_accumulator < tick {
                continue; // 还没到移动时间
            }
            snake.move_accumulator -= tick;
            
            // 移动
            let move_result = snake.move_forward(wrap);
            
            match move_result {
                AIMoveResult::Normal(new_head) => {
                    // 检查是否撞到玩家身体
                    if player_body.contains(&new_head) {
                        if !player_has_immunity {
                            // 玩家没有免疫，AI蛇死亡
                            dead_indices.push(i);
                            continue;
                        }
                    }
                    
                    // 检查是否撞到其他 AI 蛇
                    for (j, other_body) in bodies.iter().enumerate() {
                        if i != j && other_body.contains(&new_head) {
                            dead_indices.push(i);
                            break;
                        }
                    }
                    
                    if dead_indices.contains(&i) {
                        continue;
                    }
                    
                    // 检查是否吃到普通食物
                    if new_head == *food {
                        snake.grow();
                        need_respawn_food = true;
                    }
                    
                    // 检查是否吃到掉落的食物
                    let mut eaten_dropped = Vec::new();
                    for (idx, dropped) in self.dropped_foods.iter().enumerate() {
                        if new_head == dropped.pos {
                            snake.grow();
                            eaten_dropped.push(idx);
                        }
                    }
                    for idx in eaten_dropped.into_iter().rev() {
                        self.dropped_foods.remove(idx);
                    }
                    
                    // 检查是否吃到果实
                    if let Some(idx) = fruits.iter().position(|f| f.pos == new_head) {
                        let fruit = fruits.remove(idx);
                        
                        // AI 蛇也会受到果实效果（简化处理）
                        if let Some(config) = registry.get_config(fruit.type_id) {
                            match config.category {
                                FruitCategory::Normal => {
                                    // 普通果实：增长
                                    snake.grow();
                                }
                                FruitCategory::Trap => {
                                    // 陷阱果实：应用负面效果
                                    match fruit.type_id {
                                        "freeze" => snake.buff_state.frozen = true,
                                        "slow" => snake.buff_state.activate_slow(),
                                        "dizzy" => snake.buff_state.activate_dizzy(),
                                        "slime" => snake.buff_state.activate_slime(),
                                        _ => {} // trap 果实对 AI 蛇无效
                                    }
                                }
                                FruitCategory::Power => {
                                    // 功能果实：应用正面效果
                                    match fruit.type_id {
                                        "shield" => snake.buff_state.activate_shield(),
                                        "speed" => snake.buff_state.activate_speed(),
                                        "ghost" => snake.buff_state.activate_ghost(),
                                        _ => snake.grow(), // 其他功能果实当作普通果实
                                    }
                                }
                                FruitCategory::Special => {
                                    // 特殊果实：AI 蛇只增长
                                    snake.grow();
                                }
                            }
                        }
                    }
                }
                AIMoveResult::WallCollision | AIMoveResult::SelfCollision => {
                    dead_indices.push(i);
                }
                AIMoveResult::Frozen => {
                    // 冰冻状态，不移动
                }
            }
        }
        
        // 如果有 AI 蛇吃到了食物，重新生成食物
        if need_respawn_food {
            *food = Self::spawn_new_food(player_body, &self.snakes, rng);
        }
        
        // 检查玩家是否撞到 AI 蛇身体
        if !player_body.is_empty() {
            let player_head = player_body[0];
            for snake in &self.snakes {
                // 检查是否撞到 AI 蛇身体（不包括头，头对头另外处理）
                if snake.body.iter().skip(1).any(|&p| p == player_head) {
                    if !player_has_immunity {
                        result.player_died = true;
                    }
                }
                // 头对头碰撞
                if snake.head() == player_head {
                    if !player_has_immunity {
                        result.player_died = true;
                    }
                }
            }
        }
        
        // 处理死亡的 AI 蛇
        dead_indices.sort_unstable();
        dead_indices.dedup();
        for idx in dead_indices.into_iter().rev() {
            let dead_snake = self.snakes.remove(idx);
            
            // 生成死亡粒子效果
            for &pos in &dead_snake.body {
                for _ in 0..3 {
                    let lifetime = 0.5;
                    particles.push(Particle {
                        pos: vec2(pos.x as f32 * 20.0 + 10.0, pos.y as f32 * 20.0 + 10.0),
                        vel: vec2(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)),
                        color: dead_snake.color,
                        lifetime,
                        max_lifetime: lifetime,
                        size: rng.gen_range(2.0..5.0),
                    });
                }
            }
            
            // 掉落食物
            for &pos in &dead_snake.body {
                // 随机偏移 -1 到 1
                let offset = ivec2(
                    rng.gen_range(-1..=1),
                    rng.gen_range(-1..=1),
                );
                let drop_pos = ivec2(
                    (pos.x + offset.x).clamp(0, crate::constants::GRID_W as i32 - 1),
                    (pos.y + offset.y).clamp(0, crate::constants::GRID_H as i32 - 1),
                );
                
                self.dropped_foods.push(DroppedFood {
                    pos: drop_pos,
                    spawn_time: game_time,
                    lifetime: 10.0,
                });
            }
            
            result.ai_deaths += 1;
        }
        
        result
    }
    
    /// 更新掉落的食物（移除过期的）
    pub fn update_dropped_foods(&mut self, game_time: f32) {
        self.dropped_foods.retain(|f| game_time - f.spawn_time < f.lifetime);
    }
    
    /// 沙虫模式吞噬 AI 蛇
    pub fn sandworm_devour<R: Rng>(
        &mut self,
        sandworm_positions: &[IVec2],
        particles: &mut Vec<Particle>,
        rng: &mut R,
    ) -> u32 {
        let mut score_bonus = 0u32;
        let mut devoured_indices: Vec<usize> = Vec::new();
        
        for (i, snake) in self.snakes.iter().enumerate() {
            // 检查 AI 蛇是否被沙虫吞噬
            for &pos in &snake.body {
                if sandworm_positions.contains(&pos) {
                    devoured_indices.push(i);
                    score_bonus += snake.len() as u32 * 2; // 每节 2 分
                    
                    // 生成吞噬粒子效果
                    for &body_pos in &snake.body {
                        for _ in 0..2 {
                            let lifetime = 0.3;
                            particles.push(Particle {
                                pos: vec2(body_pos.x as f32 * 20.0 + 10.0, body_pos.y as f32 * 20.0 + 10.0),
                                vel: vec2(rng.gen_range(-30.0..30.0), rng.gen_range(-30.0..30.0)),
                                color: Color::new(0.8, 0.6, 0.2, 1.0), // 沙色
                                lifetime,
                                max_lifetime: lifetime,
                                size: rng.gen_range(3.0..6.0),
                            });
                        }
                    }
                    break;
                }
            }
        }
        
        // 移除被吞噬的 AI 蛇（不掉落食物）
        devoured_indices.sort_unstable();
        devoured_indices.dedup();
        for idx in devoured_indices.into_iter().rev() {
            self.snakes.remove(idx);
        }
        
        score_bonus
    }
    
    /// 生成新的食物位置（避开所有蛇）
    fn spawn_new_food<R: Rng>(
        player_body: &[IVec2],
        ai_snakes: &[AISnake],
        rng: &mut R,
    ) -> IVec2 {
        loop {
            let pos = ivec2(
                rng.gen_range(0..crate::constants::GRID_W as i32),
                rng.gen_range(0..crate::constants::GRID_H as i32),
            );
            
            if player_body.contains(&pos) {
                continue;
            }
            
            let mut occupied = false;
            for snake in ai_snakes {
                if snake.body.contains(&pos) {
                    occupied = true;
                    break;
                }
            }
            
            if !occupied {
                return pos;
            }
        }
    }
    
    /// 检查玩家是否吃到掉落的食物
    pub fn check_player_eat_dropped(&mut self, player_head: IVec2) -> u32 {
        let mut eaten = 0u32;
        self.dropped_foods.retain(|f| {
            if f.pos == player_head {
                eaten += 1;
                false
            } else {
                true
            }
        });
        eaten
    }
}

/// AI 更新结果
#[derive(Default)]
pub struct AIUpdateResult {
    /// 玩家是否死亡
    pub player_died: bool,
    /// AI 蛇死亡数量
    pub ai_deaths: u32,
}
