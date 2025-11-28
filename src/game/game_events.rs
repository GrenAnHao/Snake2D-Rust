//! 游戏事件系统
//!
//! 提供声明式的事件处理机制，避免在主循环中硬编码特殊逻辑

use macroquad::prelude::*;

/// 游戏事件
#[derive(Clone, Debug)]
pub enum GameEvent {
    /// 生成 AI 蛇
    SpawnAISnake,
    /// 播放音效
    PlaySound(SoundType),
    /// 添加分数
    AddScore(u32),
    /// 游戏结束
    GameOver,
}

/// 音效类型
#[derive(Clone, Copy, Debug)]
pub enum SoundType {
    Eat,
    Trap,
    Power,
    GameOver,
}

/// 事件队列
#[derive(Default)]
pub struct EventQueue {
    events: Vec<GameEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    /// 推送事件
    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    /// 获取并清空所有事件
    pub fn drain(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    /// 检查是否有事件
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// 过期果实处理结果
pub struct ExpireResult {
    /// 需要生成的 AI 蛇数量
    pub ai_snakes_to_spawn: u32,
    /// 其他事件
    pub events: Vec<GameEvent>,
}

impl Default for ExpireResult {
    fn default() -> Self {
        ExpireResult {
            ai_snakes_to_spawn: 0,
            events: Vec::new(),
        }
    }
}

/// 处理过期果实
/// 
/// 根据果实类型触发相应的事件
pub fn handle_expired_fruits(expired_type_ids: &[&str]) -> ExpireResult {
    let mut result = ExpireResult::default();

    for type_id in expired_type_ids {
        match *type_id {
            "snake_egg" => {
                // 蛇蛋过期时生成 AI 蛇
                result.ai_snakes_to_spawn += 1;
            }
            // 可以在这里添加其他果实的过期处理
            _ => {}
        }
    }

    result
}
