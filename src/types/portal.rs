//! 传送门相关类型定义

use macroquad::prelude::*;

/// 传送门存在时间（秒）
pub const PORTAL_LIFETIME: f32 = 20.0;

/// 传送门结构体
///
/// 成对出现的传送门，蛇进入一个会从另一个出来
#[derive(Clone)]
pub struct Portal {
    /// 传送门A位置
    pub pos_a: IVec2,
    /// 传送门B位置
    pub pos_b: IVec2,
    /// 传送门颜色
    pub color: Color,
    /// 生成时间
    pub spawn_time: f32,
    /// 存在时长
    pub lifetime: f32,
}
