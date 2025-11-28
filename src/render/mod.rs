//! # 渲染模块
//!
//! 提供所有游戏元素的绘制功能，与游戏逻辑完全分离。
//!
//! ## 模块结构
//!
//! ```text
//! render/
//! ├── snake_renderer.rs     # 蛇渲染（含各种Buff视觉效果）
//! ├── fruit_renderer.rs     # 果实渲染
//! ├── effect_renderer.rs    # 粒子、血迹、传送门、残影
//! ├── hud_renderer.rs       # 分数、Combo、Buff状态
//! └── sandworm_renderer.rs  # 沙虫变身动画
//! ```
//!
//! ## 设计原则
//!
//! 1. **纯渲染**: 只负责绘制，不修改游戏状态
//! 2. **参数化**: 所有状态通过参数传入
//! 3. **可组合**: 各渲染函数可独立调用
//!
//! ## 渲染顺序
//!
//! 正确的渲染顺序很重要，后绘制的会覆盖先绘制的：
//!
//! ```text
//! 1. draw_background()      # 清屏
//! 2. draw_border_and_grid() # 边框和网格线
//! 3. draw_blood_stains()    # 血迹（最底层）
//! 4. draw_portals()         # 传送门
//! 5. draw_food()            # 食物
//! 6. draw_fruits()          # 果实
//! 7. draw_afterimages()     # 残影
//! 8. draw_shield_effect()   # 护盾光环
//! 9. draw_snake()           # 蛇（主体）
//! 10. draw_particles()      # 粒子（最顶层）
//! 11. draw_overlay()        # 覆盖层（暂停/结束）
//! 12. draw_hud()            # HUD（始终最顶）
//! ```
//!
//! ## 坐标系统
//!
//! - 网格坐标: `IVec2`，范围 (0,0) 到 (GRID_W-1, GRID_H-1)
//! - 像素坐标: `f32`，网格坐标 * CELL
//! - 原点在左上角，Y轴向下

pub mod snake_renderer;
pub mod fruit_renderer;
pub mod effect_renderer;
pub mod hud_renderer;
pub mod sandworm_renderer;
pub mod ai_snake_renderer;

pub use snake_renderer::*;
pub use fruit_renderer::*;
pub use effect_renderer::*;
pub use hud_renderer::*;
pub use sandworm_renderer::*;
pub use ai_snake_renderer::*;

use macroquad::prelude::*;
use crate::constants::{GRID_W, GRID_H, CELL};

/// 绘制背景
pub fn draw_background() {
    clear_background(DARKGRAY);
}

/// 绘制边框和网格
pub fn draw_border_and_grid() {
    let w = GRID_W as f32 * CELL;
    let h = GRID_H as f32 * CELL;
    draw_rectangle_lines(0.0, 0.0, w, h, 2.0, WHITE);

    let gc = Color { r: 0.6, g: 0.6, b: 0.6, a: 0.2 };
    for x in 1..GRID_W {
        let xf = x as f32 * CELL;
        draw_line(xf, 0.0, xf, h, 1.0, gc);
    }
    for y in 1..GRID_H {
        let yf = y as f32 * CELL;
        draw_line(0.0, yf, w, yf, 1.0, gc);
    }
}
