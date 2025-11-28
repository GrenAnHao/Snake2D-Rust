//! 炸弹状态相关类型定义

use macroquad::prelude::*;

/// 炸弹在蛇体内的状态
#[derive(Clone, Default)]
pub struct BombState {
    /// 是否有炸弹在体内
    pub active: bool,
    /// 炸弹当前在蛇身的位置索引（0=头部）
    pub position: usize,
    /// 炸弹移动计时器
    pub move_timer: f32,
    /// 每格移动间隔（秒）
    pub move_interval: f32,
}

impl BombState {
    /// 激活炸弹（刚吞食时）
    pub fn activate(&mut self) {
        self.active = true;
        self.position = 0;
        self.move_timer = 0.0;
        self.move_interval = 0.3; // 每0.3秒移动一格
    }
    
    /// 清除炸弹状态
    pub fn clear(&mut self) {
        self.active = false;
        self.position = 0;
        self.move_timer = 0.0;
    }
    
    /// 计算闪烁频率（越接近爆炸越快）
    pub fn flash_frequency(&self, snake_len: usize) -> f32 {
        if snake_len == 0 {
            return 2.0;
        }
        let progress = self.position as f32 / (snake_len as f32 / 2.0);
        2.0 + progress.min(1.0) * 8.0 // 2Hz ~ 10Hz
    }
    
    /// 检查是否应该爆炸（到达蛇身一半位置）
    pub fn should_explode(&self, snake_len: usize) -> bool {
        self.active && self.position >= snake_len / 2
    }
}

/// 炸弹爆炸后遗症状态
#[derive(Clone, Default)]
pub struct BombAfterEffect {
    /// 是否在后遗症期间
    pub active: bool,
    /// 剩余时间
    pub timer: f32,
    /// 掉血计时器
    pub bleed_timer: f32,
    /// 无法增长标记
    pub no_grow: bool,
    /// 撕裂位置索引（用于渲染撕裂纹理）
    pub torn_index: usize,
}

impl BombAfterEffect {
    /// 激活后遗症
    pub fn activate(&mut self, torn_index: usize) {
        self.active = true;
        self.timer = 20.0; // 20秒后遗症
        self.bleed_timer = 0.0;
        self.no_grow = false; // 可以进食增长
        self.torn_index = torn_index;
    }
    
    /// 清除后遗症（被恢复果实治愈）
    pub fn clear(&mut self) {
        self.active = false;
        self.timer = 0.0;
        self.bleed_timer = 0.0;
        self.no_grow = false;
        self.torn_index = 0;
    }
    
    /// 更新后遗症状态
    /// 返回是否应该掉血
    pub fn update(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }
        
        self.timer -= dt;
        if self.timer <= 0.0 {
            self.clear();
            return false;
        }
        
        // 每0.5秒掉一节血
        self.bleed_timer += dt;
        if self.bleed_timer >= 0.5 {
            self.bleed_timer -= 0.5;
            return true;
        }
        
        false
    }
}

/// 掉落的蛇身肉块（炸断的尾巴）
#[derive(Clone)]
pub struct DroppedMeat {
    /// 肉块位置
    pub pos: IVec2,
    /// 生成时间
    pub spawn_time: f32,
    /// 生命周期
    pub lifetime: f32,
}
