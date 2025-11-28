//! # Snake 2D 游戏库
//!
//! 一个模块化的贪吃蛇游戏引擎，提供可扩展的游戏核心功能。
//!
//! ## 架构概述
//!
//! 本库采用分层架构设计，各模块职责清晰：
//!
//! - **constants**: 游戏常量配置（网格尺寸、Buff持续时间等）
//! - **types**: 核心数据结构定义（游戏状态、Buff、粒子等）
//! - **game**: 游戏逻辑（蛇移动、碰撞检测、生成逻辑）
//! - **render**: 渲染系统（蛇、果实、特效、HUD）
//! - **audio**: 音效系统（程序化WAV生成）
//! - **fruits**: 可扩展果实系统（Trait驱动设计）
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! use rtest::fruits::create_fruit_registry;
//! use rtest::game::Snake;
//! use rtest::audio::SoundManager;
//!
//! // 创建果实注册表
//! let registry = create_fruit_registry();
//!
//! // 创建蛇实例
//! let mut snake = Snake::new();
//!
//! // 移动蛇
//! let result = snake.move_forward(true, false);
//! ```
//!
//! ## 扩展果实系统
//!
//! 实现 `FruitBehavior` Trait 即可添加新果实类型：
//!
//! ```rust,ignore
//! use rtest::fruits::{FruitBehavior, FruitConfig, FruitContext, ConsumeResult};
//!
//! struct MyFruit { config: FruitConfig }
//!
//! impl FruitBehavior for MyFruit {
//!     fn config(&self) -> &FruitConfig { &self.config }
//!     fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
//!         ConsumeResult::AddScore(100)
//!     }
//! }
//! ```

/// 游戏常量配置
///
/// 包含网格尺寸、Buff持续时间、果实生命周期等常量
pub mod constants;

/// 核心数据类型
///
/// 定义游戏中使用的所有数据结构：
/// - `GameState`: 游戏状态枚举
/// - `BuffState`: Buff状态管理
/// - `Particle`: 粒子特效
/// - `Portal`: 传送门
/// - `Fruit`: 果实实例
pub mod types;

/// 可扩展果实系统
///
/// 基于 Trait 的果实系统，支持：
/// - 自定义果实类型
/// - 权重随机生成
/// - 解锁条件
/// - 自定义渲染
pub mod fruits;

/// 游戏核心逻辑
///
/// 包含：
/// - `Snake`: 蛇状态和移动
/// - 碰撞检测函数
/// - 生成逻辑
/// - Buff管理
/// - 受伤动画
pub mod game;

/// 渲染系统
///
/// 提供所有游戏元素的绘制功能：
/// - 蛇渲染（含各种Buff视觉效果）
/// - 果实渲染
/// - 粒子和特效
/// - HUD和覆盖层
pub mod render;

/// 音效系统
///
/// 程序化生成WAV音效，无需外部音频文件
pub mod audio;
