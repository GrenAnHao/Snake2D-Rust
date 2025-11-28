# 扩展指南

本文档详细介绍如何扩展 Snake 2D 游戏的各个系统。

## 目录

1. [添加新果实](#添加新果实)
2. [添加新Buff效果](#添加新buff效果)
3. [添加新渲染效果](#添加新渲染效果)
4. [添加新音效](#添加新音效)
5. [修改游戏参数](#修改游戏参数)
6. [AI蛇系统](#ai蛇系统)

---

## 添加新果实

### 步骤 1: 创建果实文件

在 `src/fruits/` 目录下创建新文件，例如 `src/fruits/power/magnet_fruit.rs`:

```rust
//! 磁铁果实 - 吸引附近的食物

use macroquad::prelude::*;
use crate::constants::CELL;
use crate::fruits::{FruitBehavior, FruitConfig, FruitCategory, FruitContext, ConsumeResult};

/// 磁铁果实
/// 
/// 效果：激活磁铁模式，自动吸引附近的食物
pub struct MagnetFruit {
    config: FruitConfig,
}

impl MagnetFruit {
    pub fn new() -> Self {
        MagnetFruit {
            config: FruitConfig {
                // 唯一标识符，用于查找和引用
                id: "magnet",
                
                // 显示名称
                name: "磁铁果实",
                
                // 类别：功能果实（正面效果）
                category: FruitCategory::Power,
                
                // 基础颜色：银灰色
                color: Color { r: 0.7, g: 0.7, b: 0.8, a: 1.0 },
                
                // 存在时长：8秒后消失
                lifetime: 8.0,
                
                // 生成权重：10（与其他功能果实相同）
                spawn_weight: 10,
                
                // 解锁条件：蛇长度达到15
                unlock_length: 15,
                
                // 不受Buff免疫影响（正面效果）
                immune_to_buffs: false,
            },
        }
    }
}

impl FruitBehavior for MagnetFruit {
    fn config(&self) -> &FruitConfig {
        &self.config
    }

    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult {
        // 激活磁铁效果（需要在 BuffState 中添加对应字段）
        // ctx.buff_state.activate_magnet();
        
        // 生成吸引粒子效果
        let center = vec2(
            ctx.fruit_pos.x as f32 * CELL + CELL / 2.0,
            ctx.fruit_pos.y as f32 * CELL + CELL / 2.0
        );
        
        // 生成银色粒子
        for _ in 0..20 {
            let angle = ctx.rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = ctx.rng.gen_range(50.0..100.0);
            ctx.particles.push(crate::types::Particle {
                pos: center,
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                color: Color { r: 0.8, g: 0.8, b: 0.9, a: 1.0 },
                lifetime: 0.5,
                max_lifetime: 0.5,
                size: 4.0,
            });
        }
        
        ConsumeResult::Continue
    }

    fn render(&self, x: f32, y: f32, time: f32) {
        // 磁铁形状渲染
        let pulse = (time * 3.0).sin() * 0.15 + 0.85;
        
        // 主体（U形磁铁简化为方块）
        let body_color = Color { 
            r: 0.7 * pulse, 
            g: 0.7 * pulse, 
            b: 0.8, 
            a: 1.0 
        };
        draw_rectangle(x, y, CELL, CELL, body_color);
        
        // 红色端点
        let red = Color { r: 0.9, g: 0.2, b: 0.2, a: 1.0 };
        draw_rectangle(x, y, CELL * 0.3, CELL * 0.4, red);
        
        // 蓝色端点
        let blue = Color { r: 0.2, g: 0.2, b: 0.9, a: 1.0 };
        draw_rectangle(x + CELL * 0.7, y, CELL * 0.3, CELL * 0.4, blue);
        
        // 磁力线效果
        let line_alpha = (time * 5.0).sin() * 0.3 + 0.5;
        let line_color = Color { r: 1.0, g: 1.0, b: 1.0, a: line_alpha };
        draw_rectangle(x + CELL * 0.4, y + CELL * 0.3, CELL * 0.2, 2.0, line_color);
    }
}
```

### 步骤 2: 导出模块

在 `src/fruits/power/mod.rs` 中添加:

```rust
pub mod magnet_fruit;
pub use magnet_fruit::MagnetFruit;
```

### 步骤 3: 注册果实

在 `src/fruits/mod.rs` 的 `create_fruit_registry()` 函数中添加:

```rust
use power::MagnetFruit;

pub fn create_fruit_registry() -> FruitRegistry {
    let mut registry = FruitRegistry::new();
    // ... 其他果实
    registry.register(Box::new(MagnetFruit::new()));
    registry
}
```

---

## 添加新Buff效果

### 步骤 1: 在 BuffState 中添加字段

编辑 `src/types/buff.rs`:

```rust
pub struct BuffState {
    // ... 现有字段
    
    // === 磁铁模式 ===
    pub magnet_active: bool,
    pub magnet_timer: f32,
    pub magnet_range: f32,  // 吸引范围
}

impl Default for BuffState {
    fn default() -> Self {
        BuffState {
            // ... 现有字段
            magnet_active: false,
            magnet_timer: 0.0,
            magnet_range: 5.0,  // 5格范围
        }
    }
}
```

### 步骤 2: 添加更新逻辑

编辑 `src/game/buff_manager.rs`:

```rust
// 在常量文件中添加
pub const MAGNET_DURATION: f32 = 8.0;

impl BuffState {
    pub fn update(&mut self, dt: f32) {
        // ... 现有逻辑
        
        // 磁铁
        if self.magnet_active {
            self.magnet_timer -= dt;
            if self.magnet_timer <= 0.0 {
                self.magnet_active = false;
                self.magnet_timer = 0.0;
            }
        }
    }

    pub fn activate_magnet(&mut self) {
        self.magnet_active = true;
        self.magnet_timer = MAGNET_DURATION;
    }
}
```

### 步骤 3: 在游戏逻辑中使用

编辑 `src/snake2d_v2.rs`，在游戏更新循环中添加磁铁效果:

```rust
// 磁铁效果：移动食物向蛇头靠近
if world.buff_state.magnet_active {
    let head = world.snake.head();
    let range = world.buff_state.magnet_range;
    
    // 检查食物是否在范围内
    let dx = world.food.x - head.x;
    let dy = world.food.y - head.y;
    let dist = ((dx * dx + dy * dy) as f32).sqrt();
    
    if dist <= range && dist > 1.0 {
        // 向蛇头移动一格
        if dx.abs() > dy.abs() {
            world.food.x -= dx.signum();
        } else {
            world.food.y -= dy.signum();
        }
    }
}
```

---

## 添加新渲染效果

### 步骤 1: 创建渲染函数

在 `src/render/effect_renderer.rs` 中添加:

```rust
/// 绘制磁铁吸引效果
/// 
/// 在蛇头周围绘制磁力线
pub fn draw_magnet_effect(head: IVec2, range: f32, time: f32) {
    let cx = head.x as f32 * CELL + CELL / 2.0;
    let cy = head.y as f32 * CELL + CELL / 2.0;
    let radius = range * CELL;
    
    // 绘制多条旋转的磁力线
    let line_count = 8;
    for i in 0..line_count {
        let base_angle = (i as f32 / line_count as f32) * std::f32::consts::TAU;
        let angle = base_angle + time * 2.0;  // 旋转动画
        
        let inner_r = CELL;
        let outer_r = radius;
        
        let x1 = cx + angle.cos() * inner_r;
        let y1 = cy + angle.sin() * inner_r;
        let x2 = cx + angle.cos() * outer_r;
        let y2 = cy + angle.sin() * outer_r;
        
        // 渐变透明度
        let alpha = (time * 4.0 + i as f32).sin() * 0.3 + 0.5;
        let color = Color { r: 0.5, g: 0.5, b: 1.0, a: alpha };
        
        draw_line(x1, y1, x2, y2, 2.0, color);
    }
    
    // 绘制范围圆
    let circle_alpha = (time * 3.0).sin() * 0.2 + 0.3;
    let circle_color = Color { r: 0.5, g: 0.5, b: 1.0, a: circle_alpha };
    draw_circle_lines(cx, cy, radius, 1.0, circle_color);
}
```

### 步骤 2: 在渲染循环中调用

在 `src/snake2d_v2.rs` 的渲染部分添加:

```rust
// 绘制磁铁效果
if world.buff_state.magnet_active {
    draw_magnet_effect(
        world.snake.head(),
        world.buff_state.magnet_range,
        world.game_time
    );
}
```

---

## 添加新音效

### 步骤 1: 在 SoundManager 中添加

编辑 `src/audio/sound_manager.rs`:

```rust
pub struct SoundManager {
    // ... 现有字段
    pub magnet_sound: Sound,
}

impl SoundManager {
    pub async fn new() -> Self {
        // ... 现有音效
        
        // 磁铁音效：中高频，较长，有"嗡嗡"感
        let magnet_sound = load_sound_from_bytes(
            &make_tone_wav(550.0, 0.25, 0.5)
        ).await.unwrap();
        
        SoundManager {
            // ... 现有字段
            magnet_sound,
        }
    }

    pub fn play_magnet(&self) {
        play_sound(&self.magnet_sound, PlaySoundParams { 
            looped: false, 
            volume: 0.6 
        });
    }
}
```

---

## 修改游戏参数

### 游戏区域

编辑 `src/constants.rs`:

```rust
// 更大的游戏区域
pub const CELL: f32 = 15.0;   // 更小的格子
pub const GRID_W: i32 = 48;   // 更宽
pub const GRID_H: i32 = 32;   // 更高
```

### 难度调整

```rust
// 更短的Buff持续时间 = 更难
pub const SHIELD_DURATION: f32 = 5.0;   // 原来是 10.0
pub const FREEZE_DURATION: f32 = 3.0;   // 原来是 2.0

// 更长的果实存在时间 = 更容易
pub const TRAP_LIFETIME: f32 = 8.0;     // 原来是 5.0
```

### 蛇速度

在 `src/snake2d_v2.rs` 中修改 tick 计算:

```rust
// 基础速度更快
let mut tick = ((100u32.saturating_sub((len / 3) * 5)).max(30)) as f32 / 1000.0;

// 或者固定速度
let tick = 0.08;  // 80ms 每步
```

---

## 测试新功能

添加新功能后，建议编写属性测试验证正确性:

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_magnet_timer_decreases(
            initial_timer in 0.1f32..20.0,
            dt in 0.01f32..5.0,
        ) {
            let mut buff = BuffState::default();
            buff.magnet_active = true;
            buff.magnet_timer = initial_timer;

            buff.update(dt);

            let expected = (initial_timer - dt).max(0.0);
            prop_assert!((buff.magnet_timer - expected).abs() < 0.0001);
        }
    }
}
```

运行测试:

```bash
cargo test
```

---

## AI蛇系统

### 概述

AI 蛇系统为游戏增加了竞争元素。玩家需要：
- 避开 AI 蛇的身体
- 利用自己的身体阻挡 AI 蛇
- 击杀 AI 蛇获取掉落的食物

### 触发方式

1. **蛇蛋果实**：吃到蛇蛋后生成一条 AI 蛇
2. **幸运方块**：负面效果之一是生成 AI 蛇

### 核心文件

```
src/game/
├── ai_snake.rs      # AI蛇结构体和行为
├── ai_manager.rs    # AI蛇管理器

src/render/
└── ai_snake_renderer.rs  # AI蛇渲染

src/fruits/special/
└── snake_egg.rs     # 蛇蛋果实
```

### AI 蛇行为

```rust
// AI 决策逻辑（每 0.2 秒执行）
impl AISnake {
    pub fn think(&mut self, food: IVec2, fruits: &[Fruit], ...) {
        // 1. 寻找最近的目标
        let target = find_nearest_target(food, fruits);
        
        // 2. 计算期望方向
        let desired_dir = calculate_direction_to(target);
        
        // 3. 检查安全性
        let safe_dir = find_safe_direction(desired_dir, obstacles);
        
        // 4. 更新方向
        self.dir = safe_dir;
    }
}
```

### 碰撞规则

| 碰撞类型 | 结果 |
|---------|------|
| AI蛇头 → 玩家身体 | AI蛇死亡，掉落食物 |
| AI蛇头 → 其他AI蛇 | AI蛇死亡，掉落食物 |
| AI蛇头 → 墙壁 | AI蛇死亡，掉落食物 |
| 玩家蛇头 → AI蛇身体 | 玩家死亡（除非有护盾/幽灵） |
| 沙虫 → AI蛇 | AI蛇被吞噬（不掉落） |

### 扩展 AI 蛇

#### 添加新的 AI 行为

编辑 `src/game/ai_snake.rs`:

```rust
impl AISnake {
    // 添加追击玩家的行为
    pub fn think_aggressive(&mut self, player_head: IVec2, ...) {
        // 有一定概率追击玩家
        if rng.gen_bool(0.3) {
            self.target = Some(player_head);
        }
    }
}
```

#### 添加 AI 蛇类型

```rust
pub enum AISnakeType {
    Normal,      // 普通：寻找食物
    Aggressive,  // 攻击型：追击玩家
    Coward,      // 胆小型：躲避玩家
}

impl AISnake {
    pub fn new_with_type(snake_type: AISnakeType, ...) -> Self {
        // 根据类型设置不同的行为参数
    }
}
```

#### 修改 AI 蛇颜色

编辑 `src/game/ai_snake.rs`:

```rust
pub const AI_COLORS: [Color; 6] = [
    ORANGE,    // 橙色
    PURPLE,    // 紫色
    PINK,      // 粉色
    SKYBLUE,   // 天蓝
    GOLD,      // 金色
    MAGENTA,   // 洋红
];
```

### 蛇蛋果实配置

编辑 `src/fruits/special/snake_egg.rs`:

```rust
FruitConfig {
    id: "snake_egg",
    name: "蛇蛋",
    category: FruitCategory::Special,
    color: Color::new(0.95, 0.9, 0.8, 1.0),
    lifetime: 15.0,
    spawn_weight: 50,    // 生成权重
    unlock_length: 5,    // 蛇长度达到5后解锁
    immune_to_buffs: false,
}
```

### 调整 AI 蛇参数

在 `src/game/ai_manager.rs` 中:

```rust
impl AIManager {
    pub fn new() -> Self {
        AIManager {
            snakes: Vec::new(),
            max_snakes: 3,  // 最大同时存在的 AI 蛇数量
            // ...
        }
    }
}
```

在 `src/snake2d_v2.rs` 中调整生成概率:

```rust
// 蛇蛋独立生成概率
if rng.gen_bool(0.008) {  // 0.8% 概率
    // 生成蛇蛋
}
```
