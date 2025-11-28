//! Buff 管理模块
//!
//! 提供 BuffState 的更新和激活方法

use crate::types::BuffState;
use crate::constants::*;

/// BuffState 的扩展方法
impl BuffState {
    /// 更新所有 Buff 计时器
    pub fn update(&mut self, dt: f32) {
        // 护盾
        if self.shield_active {
            self.shield_timer -= dt;
            if self.shield_timer <= 0.0 {
                self.shield_active = false;
                self.shield_timer = 0.0;
            }
        }

        // 速度
        if self.speed_active {
            self.speed_timer -= dt;
            if self.speed_timer <= 0.0 {
                self.speed_active = false;
                self.speed_timer = 0.0;
            }
        }

        // 幽灵
        if self.ghost_active {
            self.ghost_timer -= dt;
            if self.ghost_timer <= 0.0 {
                self.ghost_active = false;
                self.ghost_timer = 0.0;
            }
        }

        // 冰冻
        if self.frozen {
            self.freeze_timer -= dt;
            if self.freeze_timer <= 0.0 {
                self.frozen = false;
                self.freeze_timer = 0.0;
            }
        }

        // 减速
        if self.slow_active {
            self.slow_timer -= dt;
            if self.slow_timer <= 0.0 {
                self.slow_active = false;
                self.slow_timer = 0.0;
            }
        }

        // 眩晕
        if self.dizzy_active {
            self.dizzy_timer -= dt;
            if self.dizzy_timer <= 0.0 {
                self.dizzy_active = false;
                self.dizzy_timer = 0.0;
            }
        }

        // 粘液
        if self.slime_active {
            self.slime_timer -= dt;
            if self.slime_timer <= 0.0 {
                self.slime_active = false;
                self.slime_timer = 0.0;
            }
        }
        
        // 炸弹后遗症（掉血由主循环处理）
        if self.bomb_after_effect.active {
            self.bomb_after_effect.timer -= dt;
            if self.bomb_after_effect.timer <= 0.0 {
                self.bomb_after_effect.clear();
            }
        }
    }
    
    /// 更新炸弹在体内的移动
    /// 返回 Some(爆炸位置) 如果炸弹应该爆炸
    pub fn update_bomb(&mut self, dt: f32, snake_len: usize) -> Option<usize> {
        if !self.bomb_state.active {
            return None;
        }
        
        self.bomb_state.move_timer += dt;
        if self.bomb_state.move_timer >= self.bomb_state.move_interval {
            self.bomb_state.move_timer -= self.bomb_state.move_interval;
            self.bomb_state.position += 1;
            
            // 检查是否到达爆炸位置（蛇身一半）
            if self.bomb_state.should_explode(snake_len) {
                let explode_pos = self.bomb_state.position;
                self.bomb_state.clear();
                return Some(explode_pos);
            }
        }
        
        None
    }

    /// 激活护盾
    pub fn activate_shield(&mut self) {
        self.shield_active = true;
        self.shield_timer = SHIELD_DURATION;
    }

    /// 激活速度模式
    pub fn activate_speed(&mut self) {
        self.speed_active = true;
        self.speed_timer = SPEED_DURATION;
    }

    /// 激活幽灵模式
    pub fn activate_ghost(&mut self) {
        self.ghost_active = true;
        self.ghost_timer = GHOST_DURATION;
    }

    /// 激活冰冻
    pub fn activate_freeze(&mut self) {
        if !self.has_immunity() {
            self.frozen = true;
            self.freeze_timer = FREEZE_DURATION;
        }
    }

    /// 激活减速
    pub fn activate_slow(&mut self) {
        if !self.has_immunity() {
            self.slow_active = true;
            self.slow_timer = SLOW_DURATION;
        }
    }

    /// 激活眩晕
    pub fn activate_dizzy(&mut self) {
        if !self.has_immunity() {
            self.dizzy_active = true;
            self.dizzy_timer = DIZZY_DURATION;
        }
    }

    /// 激活粘液
    pub fn activate_slime(&mut self) {
        if !self.has_immunity() {
            self.slime_active = true;
            self.slime_timer = SLIME_DURATION;
        }
    }

    /// 计算当前 tick 倍率
    pub fn tick_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        if self.speed_active {
            multiplier *= 0.5; // 速度模式：tick 减半（更快）
        }
        if self.slow_active {
            multiplier *= 2.0; // 减速：tick 加倍（更慢）
        }
        if self.slime_active {
            multiplier *= 1.5; // 粘液：tick 增加 50%
        }
        multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_activation() {
        let mut buff = BuffState::default();
        buff.activate_shield();
        assert!(buff.shield_active);
        assert_eq!(buff.shield_timer, SHIELD_DURATION);
    }

    #[test]
    fn test_buff_update_decreases_timer() {
        let mut buff = BuffState::default();
        buff.activate_shield();
        let initial = buff.shield_timer;
        buff.update(1.0);
        assert_eq!(buff.shield_timer, initial - 1.0);
    }

    #[test]
    fn test_buff_deactivates_when_timer_expires() {
        let mut buff = BuffState::default();
        buff.activate_shield();
        buff.update(SHIELD_DURATION + 1.0);
        assert!(!buff.shield_active);
        assert_eq!(buff.shield_timer, 0.0);
    }

    #[test]
    fn test_immunity_blocks_negative_effects() {
        let mut buff = BuffState::default();
        buff.activate_shield();
        buff.activate_freeze();
        assert!(!buff.frozen); // 护盾免疫冰冻
    }

    #[test]
    fn test_tick_multiplier() {
        let mut buff = BuffState::default();
        assert_eq!(buff.tick_multiplier(), 1.0);

        buff.speed_active = true;
        assert_eq!(buff.tick_multiplier(), 0.5);

        buff.slow_active = true;
        assert_eq!(buff.tick_multiplier(), 1.0); // 0.5 * 2.0

        buff.speed_active = false;
        assert_eq!(buff.tick_multiplier(), 2.0);
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: modular-migration, Property 4: Buff Timer Monotonic Decrease**
    // *For any* active buff with timer > 0 and positive delta time dt,
    // after update the timer should equal max(0, timer - dt).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_buff_timer_monotonic_decrease(
            initial_timer in 0.1f32..20.0,
            dt in 0.01f32..5.0,
        ) {
            let mut buff = BuffState::default();
            buff.shield_active = true;
            buff.shield_timer = initial_timer;

            buff.update(dt);

            let expected = (initial_timer - dt).max(0.0);
            prop_assert!(
                (buff.shield_timer - expected).abs() < 0.0001,
                "Expected timer {} but got {}",
                expected,
                buff.shield_timer
            );

            // 如果计时器归零，buff 应该被关闭
            if expected <= 0.0 {
                prop_assert!(!buff.shield_active);
            }
        }

        #[test]
        fn prop_all_buffs_decrease_correctly(
            dt in 0.01f32..2.0,
        ) {
            let mut buff = BuffState::default();

            // 激活所有 buff
            buff.activate_shield();
            buff.activate_speed();
            buff.activate_ghost();
            buff.frozen = true;
            buff.freeze_timer = FREEZE_DURATION;
            buff.slow_active = true;
            buff.slow_timer = SLOW_DURATION;
            buff.dizzy_active = true;
            buff.dizzy_timer = DIZZY_DURATION;
            buff.slime_active = true;
            buff.slime_timer = SLIME_DURATION;

            let shield_before = buff.shield_timer;
            let speed_before = buff.speed_timer;
            let ghost_before = buff.ghost_timer;
            let freeze_before = buff.freeze_timer;

            buff.update(dt);

            // 所有计时器都应该减少
            prop_assert!(buff.shield_timer < shield_before || shield_before <= dt);
            prop_assert!(buff.speed_timer < speed_before || speed_before <= dt);
            prop_assert!(buff.ghost_timer < ghost_before || ghost_before <= dt);
            prop_assert!(buff.freeze_timer < freeze_before || freeze_before <= dt);
        }
    }
}
