//! # 果实系统模块
//!
//! 提供可扩展的果实系统架构，支持通过 Trait 添加新果实类型。
//!
//! ## 架构设计
//!
//! 果实系统采用 **Trait驱动** 设计模式：
//!
//! ```text
//! ┌─────────────────┐
//! │ FruitBehavior   │  ← Trait 定义果实行为接口
//! │ (Trait)         │
//! └────────┬────────┘
//!          │ impl
//!    ┌─────┴─────┬─────────────┬─────────────┐
//!    ▼           ▼             ▼             ▼
//! ┌──────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
//! │Normal│  │TrapFruit │  │ShieldFruit│ │LuckyFruit│
//! │Fruit │  │FreezeFruit│ │SpeedFruit │ │          │
//! └──────┘  │SlowFruit │  │GhostFruit │ └──────────┘
//!           └──────────┘  └──────────┘
//! ```
//!
//! ## 核心组件
//!
//! - **FruitBehavior**: 果实行为 Trait，定义 `on_consume`、`render` 等方法
//! - **FruitConfig**: 果实静态配置（ID、类别、权重、解锁条件等）
//! - **FruitRegistry**: 果实注册表，管理所有已注册的果实类型
//! - **FruitContext**: 消费上下文，提供对游戏状态的访问
//!
//! ## 添加新果实
//!
//! 1. 创建结构体并实现 `FruitBehavior`
//! 2. 在 `create_fruit_registry()` 中注册
//!
//! ```rust,ignore
//! pub struct MyFruit { config: FruitConfig }
//!
//! impl FruitBehavior for MyFruit {
//!     fn config(&self) -> &FruitConfig { &self.config }
//!     fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
//!         ctx.buff_state.activate_shield();
//!         ConsumeResult::Continue
//!     }
//! }
//! ```

pub mod fruit_trait;
pub mod fruit_registry;
pub mod fruit_context;

#[cfg(test)]
mod fruit_registry_test;

// 果实类型分组
pub mod normal;
pub mod trap;
pub mod power;
pub mod special;

pub use fruit_trait::*;
pub use fruit_registry::*;
pub use fruit_context::*;


use normal::NormalFruit;
use trap::{TrapFruit, FreezeFruit, SlowFruit, DizzyFruit, SlimeFruit};
use power::{ShieldFruit, SpeedFruit, GhostFruit, ReverseFruit, SandwormFruit};
use special::{LuckyFruit, SnakeEggFruit};

/// 创建并初始化果实注册表
///
/// 注册所有内置果实类型
pub fn create_fruit_registry() -> FruitRegistry {
    let mut registry = FruitRegistry::new();

    // 普通果实
    registry.register(Box::new(NormalFruit::new()));

    // 陷阱果实
    registry.register(Box::new(TrapFruit::new()));
    registry.register(Box::new(FreezeFruit::new()));
    registry.register(Box::new(SlowFruit::new()));
    registry.register(Box::new(DizzyFruit::new()));
    registry.register(Box::new(SlimeFruit::new()));

    // 功能果实
    registry.register(Box::new(ShieldFruit::new()));
    registry.register(Box::new(SpeedFruit::new()));
    registry.register(Box::new(GhostFruit::new()));
    registry.register(Box::new(ReverseFruit::new()));
    registry.register(Box::new(SandwormFruit::new()));

    // 特殊果实
    registry.register(Box::new(LuckyFruit::new()));
    registry.register(Box::new(SnakeEggFruit::new()));

    registry
}
