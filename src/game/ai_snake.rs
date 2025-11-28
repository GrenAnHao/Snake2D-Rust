//! AI 蛇模块
//!
//! 管理 AI 控制的蛇，包括移动、决策、Buff 状态等。

use macroquad::prelude::*;
use ::rand::Rng;
use crate::constants::{GRID_W, GRID_H};
use crate::types::{Fruit, BuffState};

/// AI 蛇颜色池
pub const AI_COLORS: [Color; 6] = [
    ORANGE,
    PURPLE,
    PINK,
    SKYBLUE,
    GOLD,
    MAGENTA,
];

/// AI 蛇结构体
#[derive(Clone)]
pub struct AISnake {
    /// 蛇身位置列表，body[0] 是蛇头
    pub body: Vec<IVec2>,
    /// 上一帧位置（用于插值渲染）
    pub prev_body: Vec<IVec2>,
    /// 当前移动方向
    pub dir: IVec2,
    /// 蛇的颜色
    pub color: Color,
    /// Buff 状态（独立于玩家）
    pub buff_state: BuffState,
    /// 当前目标位置（食物/果实）
    pub target: Option<IVec2>,
    /// 决策计时器
    pub think_timer: f32,
    /// 移动计时器（累积时间）
    pub move_accumulator: f32,
    /// 唯一标识符
    pub id: u32,
    /// 待增长节数
    pub grow_pending: u32,
}

impl AISnake {
    /// 创建新的 AI 蛇
    pub fn new<R: Rng>(id: u32, occupied: &[IVec2], rng: &mut R) -> Option<Self> {
        // 随机选择颜色
        let color = AI_COLORS[rng.gen_range(0..AI_COLORS.len())];
        
        // 随机选择初始位置（避开已占用位置）
        let mut attempts = 0;
        let head = loop {
            if attempts > 100 {
                return None; // 找不到合适位置
            }
            let pos = ivec2(
                rng.gen_range(3..GRID_W as i32 - 3),
                rng.gen_range(3..GRID_H as i32 - 3),
            );
            if !occupied.contains(&pos) {
                break pos;
            }
            attempts += 1;
        };
        
        // 随机选择初始方向
        let directions = [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)];
        let dir = directions[rng.gen_range(0..4)];
        
        // 初始长度 3-5 节
        let initial_len = rng.gen_range(3..=5);
        let mut body = vec![head];
        for i in 1..initial_len {
            body.push(head - dir * i as i32);
        }
        
        Some(AISnake {
            prev_body: body.clone(),
            body,
            dir,
            color,
            buff_state: BuffState::default(),
            target: None,
            think_timer: 0.0,
            move_accumulator: 0.0,
            id,
            grow_pending: 0,
        })
    }
    
    /// 计算移动间隔（基于蛇长度）
    pub fn get_tick(&self) -> f32 {
        let len = self.body.len() as u32;
        // 基础 tick: 150ms，比玩家稍慢
        let base_tick = ((150u32.saturating_sub((len / 3) * 5)).max(60)) as f32 / 1000.0;
        // 应用 Buff 倍率
        base_tick * self.buff_state.tick_multiplier()
    }
    
    /// 获取蛇头位置
    pub fn head(&self) -> IVec2 {
        self.body[0]
    }
    
    /// 获取蛇长度
    pub fn len(&self) -> usize {
        self.body.len()
    }
    
    /// 增长蛇身
    pub fn grow(&mut self) {
        self.grow_pending += 1;
    }
    
    /// AI 决策：选择下一个方向
    pub fn think<R: Rng>(
        &mut self,
        food: IVec2,
        fruits: &[Fruit],
        player_body: &[IVec2],
        other_ai_bodies: &[Vec<IVec2>],
        wrap: bool,
        rng: &mut R,
    ) {
        // 冰冻状态不做决策
        if self.buff_state.frozen {
            return;
        }
        
        // 眩晕状态随机方向
        if self.buff_state.dizzy_active && rng.gen_bool(0.4) {
            let directions = [ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)];
            let valid: Vec<_> = directions.iter()
                .filter(|&&d| d != -self.dir)
                .collect();
            if !valid.is_empty() {
                self.dir = *valid[rng.gen_range(0..valid.len())];
            }
            return;
        }
        
        // 寻找最近的目标
        let head = self.head();
        let mut best_target: Option<IVec2> = None;
        let mut best_dist = i32::MAX;
        
        // 检查普通食物
        let dist = (food.x - head.x).abs() + (food.y - head.y).abs();
        if dist < best_dist {
            best_dist = dist;
            best_target = Some(food);
        }
        
        // 检查果实（只考虑非陷阱果实，但 AI 不够聪明会吃到陷阱）
        for fruit in fruits {
            let dist = (fruit.pos.x - head.x).abs() + (fruit.pos.y - head.y).abs();
            if dist < best_dist {
                best_dist = dist;
                best_target = Some(fruit.pos);
            }
        }
        
        self.target = best_target;
        
        // 根据目标选择方向
        let desired_dir = if let Some(target) = self.target {
            let dx = target.x - head.x;
            let dy = target.y - head.y;
            
            // 优先选择距离更远的轴向
            if dx.abs() > dy.abs() {
                if dx > 0 { ivec2(1, 0) } else { ivec2(-1, 0) }
            } else if dy != 0 {
                if dy > 0 { ivec2(0, 1) } else { ivec2(0, -1) }
            } else {
                self.dir
            }
        } else {
            // 没有目标，随机转向
            if rng.gen_bool(0.1) {
                let directions = [ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)];
                directions[rng.gen_range(0..4)]
            } else {
                self.dir
            }
        };
        
        // 检查期望方向是否安全
        let new_dir = self.find_safe_direction(
            desired_dir,
            player_body,
            other_ai_bodies,
            wrap,
            rng,
        );
        
        // 不能反向
        if new_dir != -self.dir {
            self.dir = new_dir;
        }
    }
    
    /// 寻找安全的移动方向
    fn find_safe_direction<R: Rng>(
        &self,
        preferred: IVec2,
        player_body: &[IVec2],
        other_ai_bodies: &[Vec<IVec2>],
        wrap: bool,
        rng: &mut R,
    ) -> IVec2 {
        let directions = [preferred, ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)];
        
        for &dir in &directions {
            if dir == -self.dir {
                continue;
            }
            
            let next_pos = self.head() + dir;
            if self.is_position_safe(next_pos, player_body, other_ai_bodies, wrap) {
                return dir;
            }
        }
        
        // 没有安全方向，随机选一个（可能会死）
        let valid: Vec<_> = directions.iter()
            .filter(|&&d| d != -self.dir)
            .collect();
        if !valid.is_empty() {
            *valid[rng.gen_range(0..valid.len())]
        } else {
            self.dir
        }
    }
    
    /// 检查位置是否安全
    fn is_position_safe(
        &self,
        pos: IVec2,
        player_body: &[IVec2],
        other_ai_bodies: &[Vec<IVec2>],
        wrap: bool,
    ) -> bool {
        let check_pos = if wrap {
            ivec2(
                pos.x.rem_euclid(GRID_W as i32),
                pos.y.rem_euclid(GRID_H as i32),
            )
        } else {
            if pos.x < 0 || pos.x >= GRID_W as i32 || pos.y < 0 || pos.y >= GRID_H as i32 {
                return false;
            }
            pos
        };
        
        // 检查自己身体（除了尾巴）
        for segment in self.body.iter().take(self.body.len().saturating_sub(1)) {
            if *segment == check_pos {
                return false;
            }
        }
        
        // 检查玩家身体
        if player_body.contains(&check_pos) {
            return false;
        }
        
        // 检查其他 AI 蛇身体
        for other_body in other_ai_bodies {
            if other_body.contains(&check_pos) {
                return false;
            }
        }
        
        true
    }
    
    /// 移动 AI 蛇
    pub fn move_forward(&mut self, wrap: bool) -> AIMoveResult {
        // 冰冻状态不移动
        if self.buff_state.frozen {
            return AIMoveResult::Frozen;
        }
        
        // 保存上一帧位置
        self.prev_body = self.body.clone();
        
        let mut new_head = self.head() + self.dir;
        
        // 处理边界
        if wrap {
            new_head.x = new_head.x.rem_euclid(GRID_W as i32);
            new_head.y = new_head.y.rem_euclid(GRID_H as i32);
        } else if new_head.x < 0 || new_head.x >= GRID_W as i32 
               || new_head.y < 0 || new_head.y >= GRID_H as i32 {
            return AIMoveResult::WallCollision;
        }
        
        // 检查自身碰撞
        if self.body.iter().take(self.body.len().saturating_sub(1)).any(|&p| p == new_head) {
            return AIMoveResult::SelfCollision;
        }
        
        // 移动
        self.body.insert(0, new_head);
        if self.grow_pending > 0 {
            self.grow_pending -= 1;
        } else {
            self.body.pop();
        }
        
        AIMoveResult::Normal(new_head)
    }
}

/// AI 蛇移动结果
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIMoveResult {
    /// 正常移动，返回新蛇头位置
    Normal(IVec2),
    /// 撞墙
    WallCollision,
    /// 撞自己
    SelfCollision,
    /// 冰冻状态
    Frozen,
}
