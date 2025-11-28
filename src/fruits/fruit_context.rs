//! 果实上下文定义
//!
//! 提供果实消费时所需的游戏状态访问

use macroquad::prelude::*;
use ::rand::rngs::ThreadRng;
use crate::types::{BuffState, DamageState, Particle, ComboState};

/// 果实上下文
///
/// 传递给果实的on_consume方法，提供对游戏状态的访问
pub struct FruitContext<'a> {
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
}
