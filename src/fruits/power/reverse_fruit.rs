//! 逆转果实实现

use macroquad::prelude::*;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};

/// 逆转果实
///
/// 粉色果实，蛇头尾互换
pub struct ReverseFruit {
    config: FruitConfig,
}

impl ReverseFruit {
    pub fn new() -> Self {
        ReverseFruit {
            config: FruitConfig {
                id: "reverse",
                name: "逆转果实",
                category: FruitCategory::Power,
                color: Color { r: 1.0, g: 0.4, b: 0.7, a: 1.0 },
                lifetime: 5.0,
                spawn_weight: 15,
                unlock_length: 10,
                immune_to_buffs: false,
            },
        }
    }
}

impl Default for ReverseFruit {
    fn default() -> Self {
        Self::new()
    }
}

impl FruitBehavior for ReverseFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 逆转蛇身
        ctx.snake.reverse();

        // 更新方向为新头部的方向
        if ctx.snake.len() > 1 {
            let new_head = ctx.snake[0];
            let second = ctx.snake[1];
            *ctx.dir = ivec2(new_head.x - second.x, new_head.y - second.y);
        }

        ConsumeResult::Continue
    }
}
