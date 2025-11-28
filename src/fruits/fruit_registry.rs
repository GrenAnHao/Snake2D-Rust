//! 果实注册表
//!
//! 管理所有已注册的果实类型

use std::collections::HashMap;
use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use macroquad::prelude::*;
use super::{FruitBehavior, FruitCategory, FruitConfig};

/// 果实实例（运行时数据）
#[derive(Clone)]
pub struct FruitInstance {
    /// 果实类型ID
    pub type_id: &'static str,
    /// 网格位置
    pub pos: IVec2,
    /// 生成时间戳
    pub spawn_time: f32,
    /// 存在时长（从配置复制）
    pub lifetime: f32,
}

/// 果实注册表
///
/// 管理所有已注册的果实类型，提供生成和查询功能
pub struct FruitRegistry {
    /// 所有已注册的果实
    fruits: HashMap<&'static str, Box<dyn FruitBehavior>>,
    /// 按类别分组的果实ID
    by_category: HashMap<FruitCategory, Vec<&'static str>>,
}

impl FruitRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        FruitRegistry {
            fruits: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// 注册果实类型
    pub fn register(&mut self, fruit: Box<dyn FruitBehavior>) {
        let config = fruit.config();
        let id = config.id;
        let category = config.category;

        // 添加到类别列表
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);

        // 添加到主映射
        self.fruits.insert(id, fruit);
    }

    /// 获取果实行为
    pub fn get(&self, id: &str) -> Option<&dyn FruitBehavior> {
        self.fruits.get(id).map(|b| b.as_ref())
    }

    /// 获取果实配置
    pub fn get_config(&self, id: &str) -> Option<&FruitConfig> {
        self.fruits.get(id).map(|b| b.config())
    }

    /// 按类别随机选择果实（考虑权重和解锁条件）
    pub fn random_by_category(
        &self,
        category: FruitCategory,
        snake_length: usize,
        rng: &mut ThreadRng,
    ) -> Option<&'static str> {
        let ids = self.by_category.get(&category)?;

        // 过滤已解锁的果实并计算权重
        let candidates: Vec<(&'static str, u32)> = ids
            .iter()
            .filter_map(|&id| {
                let config = self.get_config(id)?;
                if config.unlock_length <= snake_length {
                    Some((id, config.spawn_weight))
                } else {
                    None
                }
            })
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // 按权重随机选择
        let total_weight: u32 = candidates.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return None;
        }

        let mut roll = rng.gen_range(0..total_weight);
        for (id, weight) in &candidates {
            if roll < *weight {
                return Some(*id);
            }
            roll -= *weight;
        }

        // 兜底返回第一个
        candidates.first().map(|(id, _)| *id)
    }

    /// 获取某类别的所有果实ID
    pub fn get_by_category(&self, category: FruitCategory) -> Vec<&'static str> {
        self.by_category
            .get(&category)
            .cloned()
            .unwrap_or_default()
    }

    /// 检查是否有指定类别的果实
    pub fn has_category(&self, category: FruitCategory) -> bool {
        self.by_category
            .get(&category)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }
}

impl Default for FruitRegistry {
    fn default() -> Self {
        Self::new()
    }
}
