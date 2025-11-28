//! 受伤状态和血迹相关类型定义

use macroquad::prelude::*;

/// 受伤动画阶段枚举
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DamagePhase {
    /// 无受伤动画
    None,
    /// 红色闪烁阶段
    Flashing,
    /// 尾巴碎裂阶段
    Crumbling,
}

/// 受伤状态结构体
///
/// 管理吃到陷阱果实后的受伤动画
#[derive(Clone)]
pub struct DamageState {
    /// 是否正在播放受伤动画
    pub active: bool,
    /// 当前闪烁周期计时器
    pub flash_timer: f32,
    /// 已完成的闪烁次数
    pub flash_count: i32,
    /// 碎裂间隔计时器
    pub crumble_timer: f32,
    /// 当前碎裂到第几节尾巴
    pub crumble_index: usize,
    /// 待移除的尾巴位置列表
    pub tail_to_remove: Vec<IVec2>,
    /// 当前动画阶段
    pub phase: DamagePhase,
}

impl Default for DamageState {
    fn default() -> Self {
        DamageState {
            active: false,
            flash_timer: 0.0,
            flash_count: 0,
            crumble_timer: 0.0,
            crumble_index: 0,
            tail_to_remove: vec![],
            phase: DamagePhase::None,
        }
    }
}

/// 血迹结构体
///
/// 当蛇尾碎裂时在地面留下的血迹
#[derive(Clone)]
pub struct BloodStain {
    /// 网格位置
    pub pos: IVec2,
    /// 生成时间
    pub spawn_time: f32,
    /// 存在时长
    pub lifetime: f32,
    /// 大小系数
    pub size: f32,
    /// 初始透明度
    pub alpha: f32,
}

// 受伤动画常量
pub const DAMAGE_FLASH_DURATION: f32 = 0.6;
pub const DAMAGE_FLASH_COUNT: i32 = 4;
pub const DAMAGE_CRUMBLE_INTERVAL: f32 = 0.15;
pub const BLOOD_STAIN_LIFETIME: f32 = 5.0;
