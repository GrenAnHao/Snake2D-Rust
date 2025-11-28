//! # 果实 Trait 定义
//!
//! 定义果实的核心接口，所有果实类型都需要实现此 Trait。
//!
//! ## 设计理念
//!
//! 使用 Trait 而非枚举的原因：
//! - **开放扩展**: 添加新果实无需修改现有代码
//! - **封装行为**: 每种果实的逻辑独立封装
//! - **多态渲染**: 每种果实可以有独特的视觉效果
//!
//! ## 实现指南
//!
//! 必须实现的方法：
//! - `config()`: 返回果实的静态配置
//! - `on_consume()`: 定义被吃掉时的效果
//!
//! 可选覆盖的方法：
//! - `render()`: 自定义渲染（默认绘制纯色方块）
//! - `on_spawn()`: 生成时的效果
//! - `on_expire()`: 消失时的效果

use macroquad::prelude::*;
use crate::constants::CELL;
use super::FruitContext;

/// 果实类别
///
/// 用于分组管理和随机生成。每个类别有独立的生成逻辑和概率。
///
/// ## 类别说明
///
/// - **Normal**: 基础果实，提供分数和增长
/// - **Trap**: 陷阱果实，造成负面效果（可被护盾免疫）
/// - **Power**: 功能果实，提供增益效果
/// - **Special**: 特殊果实，效果独特或随机
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FruitCategory {
    /// 普通果实 - 基础得分和增长
    Normal,
    /// 陷阱果实 - 负面效果（冰冻、减速、受伤等）
    Trap,
    /// 功能果实 - 正面效果（护盾、速度、幽灵等）
    Power,
    /// 特殊果实 - 独特效果（幸运方块等）
    Special,
}

/// 消费结果
///
/// 定义果实被吃掉后的游戏行为。支持组合多个结果。
///
/// ## 使用示例
///
/// ```rust,ignore
/// // 简单加分
/// ConsumeResult::AddScore(10)
///
/// // 组合效果：加分 + 重置Combo
/// ConsumeResult::Multiple(vec![
///     ConsumeResult::AddScore(10),
///     ConsumeResult::ResetCombo,
/// ])
/// ```
#[derive(Clone, Debug)]
pub enum ConsumeResult {
    /// 继续游戏，无特殊效果
    Continue,
    /// 游戏结束（蛇太短时吃到陷阱）
    GameOver,
    /// 重置 Combo 计数（吃到陷阱时）
    ResetCombo,
    /// 增加指定分数
    AddScore(u32),
    /// 多个结果组合（按顺序处理）
    Multiple(Vec<ConsumeResult>),
}

/// 果实配置（静态属性）
///
/// 定义果实的不变属性，在注册时设置。
///
/// ## 字段说明
///
/// - `id`: 唯一标识符，用于查找和引用
/// - `spawn_weight`: 同类别内的生成权重，数值越大越容易生成
/// - `unlock_length`: 蛇达到此长度后才会生成该果实
/// - `immune_to_buffs`: 如果为 true，即使有护盾也会生效
/// - `weight_growth`: 每增长N格增加的权重（0表示不增长）
#[derive(Clone)]
pub struct FruitConfig {
    /// 唯一标识符
    ///
    /// 用于在注册表中查找果实。建议使用小写字母和下划线。
    /// 例如: "freeze", "shield", "lucky"
    pub id: &'static str,

    /// 显示名称
    ///
    /// 用于 UI 显示，可以包含中文或特殊字符。
    pub name: &'static str,

    /// 果实类别
    ///
    /// 决定生成逻辑和基础行为分类。
    pub category: FruitCategory,

    /// 基础颜色
    ///
    /// 默认渲染时使用的颜色。自定义 render() 可以忽略此值。
    pub color: Color,

    /// 存在时长（秒）
    ///
    /// 果实在场景中存在的时间。0 表示永久存在。
    pub lifetime: f32,

    /// 生成权重（基础值）
    ///
    /// 同类别内的相对生成概率。例如权重 10 的果实
    /// 比权重 5 的果实生成概率高一倍。
    pub spawn_weight: u32,

    /// 解锁所需蛇长
    ///
    /// 蛇的长度达到此值后才会生成该果实。
    /// 0 表示无限制，游戏开始就可能生成。
    pub unlock_length: usize,

    /// 是否无视 Buff 免疫
    ///
    /// 如果为 true，即使玩家有护盾/幽灵等免疫效果，
    /// 该果实的效果仍然会生效。
    pub immune_to_buffs: bool,

    /// 权重增长系数
    ///
    /// 每超过 unlock_length 多少格，权重增加1。
    /// 0 表示不增长（默认）。
    /// 
    /// 例如: weight_growth = 5, unlock_length = 6, spawn_weight = 5
    /// - 蛇长6: 权重 = 5 + (6-6)/5 = 5
    /// - 蛇长11: 权重 = 5 + (11-6)/5 = 6
    /// - 蛇长16: 权重 = 5 + (16-6)/5 = 7
    /// - 蛇长26: 权重 = 5 + (26-6)/5 = 9
    pub weight_growth: u32,
}

/// 果实行为 Trait（核心接口）
///
/// 所有果实类型都需要实现此 Trait。这是果实系统的核心抽象。
///
/// ## 必须实现
///
/// - `config()`: 返回果实的静态配置
/// - `on_consume()`: 定义被吃掉时的效果
///
/// ## 可选覆盖
///
/// - `render()`: 自定义渲染效果
/// - `on_spawn()`: 生成时触发的效果
/// - `on_expire()`: 超时消失时触发的效果
///
/// ## 线程安全
///
/// 要求 `Send + Sync` 以支持多线程环境（虽然当前是单线程）。
///
/// ## 示例实现
///
/// ```rust,ignore
/// pub struct MyFruit {
///     config: FruitConfig,
/// }
///
/// impl FruitBehavior for MyFruit {
///     fn config(&self) -> &FruitConfig {
///         &self.config
///     }
///
///     fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
///         // 激活护盾效果
///         ctx.buff_state.activate_shield();
///         // 增加分数
///         *ctx.score += 50;
///         ConsumeResult::Continue
///     }
///
///     fn render(&self, x: f32, y: f32, time: f32) {
///         // 脉动效果
///         let pulse = (time * 4.0).sin() * 0.2 + 0.8;
///         let color = Color { a: pulse, ..self.config.color };
///         draw_rectangle(x, y, CELL, CELL, color);
///     }
/// }
/// ```
pub trait FruitBehavior: Send + Sync {
    /// 获取果实配置
    ///
    /// 返回果实的静态配置信息。通常直接返回结构体中存储的配置。
    fn config(&self) -> &FruitConfig;

    /// 消费时的效果
    ///
    /// 当蛇头碰到果实时调用。可以修改游戏状态。
    ///
    /// # 参数
    /// - `ctx`: 果实上下文，提供对游戏状态的可变访问
    ///
    /// # 返回
    /// 消费结果，决定后续游戏行为（继续、结束、加分等）
    ///
    /// # 可用操作
    /// - 修改蛇身: `ctx.snake.push()`, `ctx.snake.pop()`
    /// - 激活Buff: `ctx.buff_state.activate_shield()`
    /// - 生成粒子: `ctx.particles.push()`
    /// - 修改分数: `*ctx.score += 10`
    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult;

    /// 自定义渲染
    ///
    /// 绘制果实的视觉效果。默认实现绘制纯色方块。
    ///
    /// # 参数
    /// - `x`, `y`: 果实左上角的像素坐标
    /// - `time`: 当前游戏时间（用于动画效果）
    ///
    /// # 默认行为
    /// 使用 `config().color` 绘制 CELL x CELL 的方块
    fn render(&self, x: f32, y: f32, _time: f32) {
        draw_rectangle(x, y, CELL, CELL, self.config().color);
    }

    /// 生成时的效果（可选）
    ///
    /// 果实刚生成时调用。可用于生成特效或播放音效。
    fn on_spawn(&self, _ctx: &mut FruitContext) {}

    /// 消失时的效果（可选）
    ///
    /// 果实因超时而消失时调用。可用于生成消失特效。
    fn on_expire(&self, _ctx: &mut FruitContext) {}

    /// 获取果实ID
    ///
    /// 便捷方法，等同于 `self.config().id`
    fn id(&self) -> &'static str {
        self.config().id
    }

    /// 获取果实类别
    ///
    /// 便捷方法，等同于 `self.config().category`
    fn category(&self) -> FruitCategory {
        self.config().category
    }
}
