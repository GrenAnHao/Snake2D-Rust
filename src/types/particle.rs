//! 粒子结构体定义

use macroquad::prelude::*;

/// 粒子结构体
///
/// 用于实现各种视觉特效，如爆炸、血液飞溅等。
///
/// ## 物理模拟
/// - 粒子按照速度移动，并受到阻尼影响逐渐减速
/// - 生命周期结束后自动移除
///
/// ## 渲染
/// - 大小和透明度随生命周期衰减
/// - 颜色可自定义
#[derive(Clone)]
pub struct Particle {
    /// 当前位置（像素坐标）
    pub pos: Vec2,
    /// 速度向量
    pub vel: Vec2,
    /// 粒子颜色
    pub color: Color,
    /// 剩余生命周期
    pub lifetime: f32,
    /// 最大生命周期（用于计算衰减）
    pub max_lifetime: f32,
    /// 粒子大小
    pub size: f32,
}
