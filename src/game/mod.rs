//! # 游戏核心模块
//!
//! 封装游戏核心逻辑，与渲染和音效完全分离。
//!
//! ## 模块结构
//!
//! ```text
//! game/
//! ├── snake.rs          # 蛇状态和移动逻辑
//! ├── collision.rs      # 碰撞检测函数
//! ├── spawn.rs          # 生成逻辑（食物、果实、传送门）
//! ├── buff_manager.rs   # Buff 计时器管理
//! ├── damage_manager.rs # 受伤动画状态机
//! └── fruit_handler.rs  # 果实消费处理
//! ```
//!
//! ## 设计原则
//!
//! 1. **纯逻辑**: 不包含任何渲染代码
//! 2. **可测试**: 所有函数都可以独立单元测试
//! 3. **属性测试**: 使用 proptest 验证核心不变量
//!
//! ## 核心类型
//!
//! - `Snake`: 蛇的状态和移动
//! - `MoveResult`: 移动结果（正常/撞墙/自撞）
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use rtest::game::{Snake, MoveResult, check_fruit_collision};
//!
//! let mut snake = Snake::new();
//! let result = snake.move_forward(true, false);
//!
//! match result {
//!     MoveResult::Normal(new_head) => {
//!         // 检查是否吃到果实
//!         if let Some(idx) = check_fruit_collision(new_head, &fruits) {
//!             // 处理果实效果
//!         }
//!     }
//!     MoveResult::WallCollision => { /* 游戏结束 */ }
//!     MoveResult::SelfCollision => { /* 游戏结束 */ }
//! }
//! ```

pub mod snake;
pub mod collision;
pub mod spawn;
pub mod buff_manager;
pub mod damage_manager;
pub mod fruit_handler;
pub mod sandworm_manager;
pub mod ai_snake;
pub mod ai_manager;
pub mod spawn_manager;
pub mod bomb_manager;
pub mod game_events;

pub use snake::*;
pub use collision::*;
pub use spawn::*;
pub use damage_manager::*;
pub use fruit_handler::*;
pub use sandworm_manager::*;
pub use ai_snake::{AISnake, AIMoveResult, AI_COLORS};
pub use ai_manager::{AIManager, AIUpdateResult, DroppedFood};
pub use spawn_manager::{FruitSpawnManager, SpawnRule, IndependentSpawnRule, create_default_spawn_manager};
pub use bomb_manager::{BombManager, BombUpdateResult};
pub use game_events::{GameEvent, SoundType, EventQueue, ExpireResult, handle_expired_fruits};
