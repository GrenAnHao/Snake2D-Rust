//! Buff状态相关类型定义

use macroquad::prelude::*;

/// 沙虫变身阶段枚举
///
/// 沙虫模式是一个复杂的多阶段动画序列
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SandwormPhase {
    /// 未激活
    None,
    /// 闪烁阶段（原地不动，身体闪烁）
    Flashing,
    /// 变色阶段（从头到尾变成沙色）
    Transforming,
    /// 退出阶段（蛇向边界移动直到消失）
    Exiting,
    /// 填充阶段（蛇不断增长填满整个游戏盘）
    Filling,
    /// 填满后闪烁变红
    FilledFlashing,
    /// 吞噬阶段（坍缩动画）
    Consuming,
}

/// Buff状态管理结构体
///
/// 集中管理所有增益/减益效果的状态
#[derive(Clone)]
pub struct BuffState {
    // === 护盾模式 ===
    pub shield_active: bool,
    pub shield_timer: f32,

    // === 沙虫模式 ===
    pub sandworm_active: bool,
    pub sandworm_path: Vec<IVec2>,
    pub sandworm_index: usize,
    pub sandworm_tick: f32,
    pub sandworm_phase: SandwormPhase,
    pub sandworm_phase_timer: f32,
    pub sandworm_transform_index: usize,
    pub sandworm_original_snake: Vec<IVec2>,
    pub sandworm_original_dir: IVec2,
    pub sandworm_collapse_center: Vec2,
    pub sandworm_collapse_progress: f32,
    pub sandworm_collapse_positions: Vec<Vec2>,
    pub sandworm_exit_dir: IVec2,

    // === 速度模式 ===
    pub speed_active: bool,
    pub speed_timer: f32,

    // === 幽灵模式 ===
    pub ghost_active: bool,
    pub ghost_timer: f32,

    // === 冰冻状态 ===
    pub frozen: bool,
    pub freeze_timer: f32,

    // === 减速状态 ===
    pub slow_active: bool,
    pub slow_timer: f32,

    // === 眩晕状态 ===
    pub dizzy_active: bool,
    pub dizzy_timer: f32,

    // === 粘液状态 ===
    pub slime_active: bool,
    pub slime_timer: f32,

    // === AI蛇生成标记 ===
    /// 幸运方块触发的 AI 蛇生成
    pub pending_ai_spawn: bool,
}

impl Default for BuffState {
    fn default() -> Self {
        BuffState {
            shield_active: false,
            shield_timer: 0.0,
            sandworm_active: false,
            sandworm_path: vec![],
            sandworm_index: 0,
            sandworm_tick: 0.0,
            sandworm_phase: SandwormPhase::None,
            sandworm_phase_timer: 0.0,
            sandworm_transform_index: 0,
            sandworm_original_snake: vec![],
            sandworm_original_dir: ivec2(1, 0),
            sandworm_collapse_center: vec2(0.0, 0.0),
            sandworm_collapse_progress: 0.0,
            sandworm_collapse_positions: vec![],
            sandworm_exit_dir: ivec2(1, 0),
            speed_active: false,
            speed_timer: 0.0,
            ghost_active: false,
            ghost_timer: 0.0,
            frozen: false,
            freeze_timer: 0.0,
            slow_active: false,
            slow_timer: 0.0,
            dizzy_active: false,
            dizzy_timer: 0.0,
            slime_active: false,
            slime_timer: 0.0,
            pending_ai_spawn: false,
        }
    }
}

impl BuffState {
    /// 检查是否有任何免疫效果激活
    pub fn has_immunity(&self) -> bool {
        self.shield_active || self.sandworm_active || self.ghost_active
    }
}
