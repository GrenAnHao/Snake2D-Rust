//! 果实上下文定义
//!
//! 提供果实消费时所需的游戏状态访问
//!
//! ## 设计原则
//!
//! FruitContext 应该足够丰富，让每种果实在自己的回调中直接完成所有操作，
//! 而不是通过事件间接触发。这样新增果实时只需要修改该果实的源文件。

use macroquad::prelude::*;
use ::rand::rngs::ThreadRng;
use crate::types::{BuffState, DamageState, Particle, ComboState, Fruit};
use crate::game::AIManager;

/// 果实上下文
///
/// 传递给果实的 on_consume / on_expire / on_spawn 方法，提供对游戏状态的访问。
///
/// ## 设计说明
///
/// 包含足够的资源引用，让果实回调可以直接操作游戏状态：
/// - 生成 AI 蛇: `ctx.ai_manager.spawn_snake()`
/// - 修改食物位置: `*ctx.food = new_pos`
/// - 生成其他果实: `ctx.fruits.push(...)`
pub struct FruitContext<'a> {
    // -------------------------------------------------------------------------
    // 核心游戏状态
    // -------------------------------------------------------------------------
    
    /// 蛇身位置列表
    pub snake: &'a mut Vec<IVec2>,
    /// 移动方向
    pub dir: &'a mut IVec2,
    /// Buff状态
    pub buff_state: &'a mut BuffState,
    /// 粒子列表
    pub particles: &'a mut Vec<Particle>,
    /// 受伤状态
    pub damage_state: &'a mut DamageState,
    /// 当前分数
    pub score: &'a mut u32,
    /// Combo状态
    pub combo_state: &'a mut ComboState,
    /// 随机数生成器
    pub rng: &'a mut ThreadRng,
    /// 当前游戏时间
    pub game_time: f32,
    /// 果实位置
    pub fruit_pos: IVec2,
    
    // -------------------------------------------------------------------------
    // 扩展资源 - 让果实可以直接操作更多游戏资源
    // -------------------------------------------------------------------------
    
    /// AI 蛇管理器 - 用于直接生成 AI 蛇（如蛇蛋孵化）
    pub ai_manager: &'a mut AIManager,
    /// 食物位置 - 用于修改食物位置
    pub food: &'a mut IVec2,
    /// 果实列表 - 用于生成或移除其他果实
    pub fruits: &'a mut Vec<Fruit>,
}
