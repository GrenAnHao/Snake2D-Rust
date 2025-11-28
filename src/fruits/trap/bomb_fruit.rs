//! 炸弹果实
//!
//! 吞食后炸弹在体内移动，到达中间位置时爆炸，将蛇拦腰炸断。

use macroquad::prelude::*;
use crate::constants::CELL;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};

/// 炸弹果实
pub struct BombFruit {
    config: FruitConfig,
}

impl BombFruit {
    pub fn new() -> Self {
        BombFruit {
            config: FruitConfig {
                id: "bomb",
                name: "炸弹",
                category: FruitCategory::Trap,
                color: Color::new(0.1, 0.1, 0.1, 1.0), // 黑色
                lifetime: 8.0,
                spawn_weight: 15, // 提高概率
                unlock_length: 4, // 蛇长度>=4才生成
                immune_to_buffs: false, // 护盾可以免疫
                weight_growth: 2, // 每增长2格，权重+1（蛇越长炸弹越多）
            },
        }
    }
}

impl Default for BombFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for BombFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 护盾免疫
        if ctx.buff_state.has_immunity() {
            return ConsumeResult::Continue;
        }
        
        // 激活炸弹状态
        ctx.buff_state.bomb_state.activate();
        
        ConsumeResult::ResetCombo
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        let cx = x + CELL / 2.0;
        let _cy = y + CELL / 2.0;
        
        // 炸弹主体（黑色圆形）
        let bomb_color = Color::new(0.15, 0.15, 0.15, 1.0);
        draw_rectangle(x + 3.0, y + 4.0, CELL - 6.0, CELL - 6.0, bomb_color);
        draw_rectangle(x + 5.0, y + 3.0, CELL - 10.0, CELL - 4.0, bomb_color);
        
        // 高光
        draw_rectangle(x + 5.0, y + 5.0, 3.0, 3.0, Color::new(0.4, 0.4, 0.4, 0.6));
        
        // 引线（棕色）
        let fuse_color = Color::new(0.5, 0.3, 0.1, 1.0);
        draw_rectangle(cx - 1.0, y + 1.0, 2.0, 4.0, fuse_color);
        
        // 引线火花（闪烁）
        let spark_alpha = (time * 10.0).sin() * 0.5 + 0.5;
        let spark_color = Color::new(1.0, 0.6, 0.0, spark_alpha);
        draw_rectangle(cx - 2.0, y, 4.0, 3.0, spark_color);
        
        // 火花粒子效果
        let spark2_alpha = (time * 15.0 + 1.0).sin() * 0.5 + 0.5;
        draw_rectangle(cx - 3.0, y - 1.0, 2.0, 2.0, Color::new(1.0, 0.8, 0.2, spark2_alpha * 0.7));
        draw_rectangle(cx + 1.0, y - 1.0, 2.0, 2.0, Color::new(1.0, 0.4, 0.1, spark2_alpha * 0.5));
    }
}
