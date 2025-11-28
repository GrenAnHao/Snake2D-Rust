//! 果实生成管理器
//!
//! 统一管理所有果实类别的生成逻辑，支持声明式配置。

use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use std::collections::HashMap;
use crate::types::Fruit;
use crate::fruits::{FruitRegistry, FruitCategory};
use super::spawn_position;

/// 生成规则配置
#[derive(Clone)]
pub struct SpawnRule {
    /// 果实类别
    pub category: FruitCategory,
    /// 生成间隔（秒），0表示每帧检查
    pub interval: f32,
    /// 生成概率（0.0-1.0）
    pub probability: f32,
    /// 最大同时存在数量，0表示无限制
    pub max_count: usize,
    /// 解锁所需蛇长，0表示无限制
    pub unlock_length: usize,
    /// 是否在沙虫模式下暂停生成
    pub pause_in_sandworm: bool,
}

impl SpawnRule {
    /// 创建新的生成规则
    pub fn new(category: FruitCategory) -> Self {
        SpawnRule {
            category,
            interval: 0.0,
            probability: 1.0,
            max_count: 0,
            unlock_length: 0,
            pause_in_sandworm: true,
        }
    }

    /// 设置生成间隔
    pub fn interval(mut self, seconds: f32) -> Self {
        self.interval = seconds;
        self
    }

    /// 设置生成概率
    pub fn probability(mut self, prob: f32) -> Self {
        self.probability = prob.clamp(0.0, 1.0);
        self
    }

    /// 设置最大数量
    pub fn max_count(mut self, count: usize) -> Self {
        self.max_count = count;
        self
    }

    /// 设置解锁长度
    pub fn unlock_length(mut self, length: usize) -> Self {
        self.unlock_length = length;
        self
    }

    /// 设置是否在沙虫模式下暂停
    pub fn pause_in_sandworm(mut self, pause: bool) -> Self {
        self.pause_in_sandworm = pause;
        self
    }
}

/// 独立果实生成规则（针对特定果实ID）
#[derive(Clone)]
pub struct IndependentSpawnRule {
    /// 果实ID
    pub fruit_id: &'static str,
    /// 生成概率（每帧）
    pub probability: f32,
    /// 最大同时存在数量
    pub max_count: usize,
    /// 解锁所需蛇长
    pub unlock_length: usize,
    /// 是否在沙虫模式下暂停生成
    pub pause_in_sandworm: bool,
}

impl IndependentSpawnRule {
    pub fn new(fruit_id: &'static str) -> Self {
        IndependentSpawnRule {
            fruit_id,
            probability: 0.01,
            max_count: 1,
            unlock_length: 0,
            pause_in_sandworm: true,
        }
    }

    pub fn probability(mut self, prob: f32) -> Self {
        self.probability = prob.clamp(0.0, 1.0);
        self
    }

    pub fn max_count(mut self, count: usize) -> Self {
        self.max_count = count;
        self
    }

    pub fn unlock_length(mut self, length: usize) -> Self {
        self.unlock_length = length;
        self
    }

    pub fn pause_in_sandworm(mut self, pause: bool) -> Self {
        self.pause_in_sandworm = pause;
        self
    }
}

/// 果实生成管理器
pub struct FruitSpawnManager {
    /// 类别生成规则
    category_rules: Vec<SpawnRule>,
    /// 类别计时器
    category_timers: HashMap<FruitCategory, f32>,
    /// 独立果实生成规则
    independent_rules: Vec<IndependentSpawnRule>,
}

impl FruitSpawnManager {
    /// 创建新的生成管理器
    pub fn new() -> Self {
        FruitSpawnManager {
            category_rules: Vec::new(),
            category_timers: HashMap::new(),
            independent_rules: Vec::new(),
        }
    }

    /// 添加类别生成规则（链式调用）
    pub fn with_category_rule(mut self, rule: SpawnRule) -> Self {
        self.category_timers.insert(rule.category, 0.0);
        self.category_rules.push(rule);
        self
    }

    /// 添加独立果实生成规则（链式调用）
    pub fn with_independent_rule(mut self, rule: IndependentSpawnRule) -> Self {
        self.independent_rules.push(rule);
        self
    }

    /// 更新并生成果实
    pub fn update(
        &mut self,
        registry: &FruitRegistry,
        snake_body: &[IVec2],
        fruits: &mut Vec<Fruit>,
        game_time: f32,
        sandworm_active: bool,
        dt: f32,
        rng: &mut ThreadRng,
    ) {
        let snake_len = snake_body.len();

        // 处理类别规则
        for rule in &self.category_rules {
            // 检查沙虫模式
            if rule.pause_in_sandworm && sandworm_active {
                continue;
            }

            // 检查解锁条件
            if snake_len < rule.unlock_length {
                continue;
            }

            // 更新计时器
            let timer = self.category_timers.entry(rule.category).or_insert(0.0);
            *timer += dt;

            // 检查间隔
            if *timer < rule.interval {
                continue;
            }
            *timer = 0.0;

            // 检查概率
            if !rng.gen_bool(rule.probability as f64) {
                continue;
            }

            // 检查最大数量
            if rule.max_count > 0 {
                let current_count = fruits
                    .iter()
                    .filter(|f| {
                        registry
                            .get_config(f.type_id)
                            .map(|c| c.category == rule.category)
                            .unwrap_or(false)
                    })
                    .count();
                if current_count >= rule.max_count {
                    continue;
                }
            }

            // 生成果实
            if let Some(pos) = spawn_position(snake_body, fruits, rng) {
                if let Some(type_id) = registry.random_by_category(rule.category, snake_len, rng) {
                    if let Some(config) = registry.get_config(type_id) {
                        fruits.push(Fruit::new(pos, type_id, game_time, config.lifetime));
                    }
                }
            }
        }

        // 处理独立果实规则
        for rule in &self.independent_rules {
            // 检查沙虫模式
            if rule.pause_in_sandworm && sandworm_active {
                continue;
            }

            // 检查解锁条件
            if snake_len < rule.unlock_length {
                continue;
            }

            // 检查最大数量
            let current_count = fruits.iter().filter(|f| f.type_id == rule.fruit_id).count();
            if current_count >= rule.max_count {
                continue;
            }

            // 检查概率
            if !rng.gen_bool(rule.probability as f64) {
                continue;
            }

            // 生成果实
            if let Some(config) = registry.get_config(rule.fruit_id) {
                if let Some(pos) = spawn_position(snake_body, fruits, rng) {
                    fruits.push(Fruit::new(pos, rule.fruit_id, game_time, config.lifetime));
                }
            }
        }
    }

    /// 重置所有计时器
    pub fn reset(&mut self) {
        for timer in self.category_timers.values_mut() {
            *timer = 0.0;
        }
    }
}

impl Default for FruitSpawnManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 创建默认的果实生成管理器
/// 
/// 包含所有内置的生成规则
pub fn create_default_spawn_manager() -> FruitSpawnManager {
    FruitSpawnManager::new()
        // 陷阱果实：每3秒40%概率生成
        .with_category_rule(
            SpawnRule::new(FruitCategory::Trap)
                .interval(3.0)
                .probability(0.4)
        )
        // 功能果实：每帧1.5%概率，最多1个，蛇长>=10解锁
        .with_category_rule(
            SpawnRule::new(FruitCategory::Power)
                .probability(0.015)
                .max_count(1)
                .unlock_length(10)
        )
        // 幸运方块：每帧0.3%概率，最多1个
        .with_independent_rule(
            IndependentSpawnRule::new("lucky")
                .probability(0.003)
                .max_count(1)
        )
        // 蛇蛋：每帧0.8%概率，最多1个，蛇长>=5解锁
        .with_independent_rule(
            IndependentSpawnRule::new("snake_egg")
                .probability(0.008)
                .max_count(1)
                .unlock_length(5)
        )
}
