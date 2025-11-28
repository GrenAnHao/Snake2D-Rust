# 架构文档

本文档详细介绍 Snake 2D 游戏的架构设计和实现原理。

## 目录

1. [整体架构](#整体架构)
2. [模块详解](#模块详解)
3. [数据流](#数据流)
4. [游戏循环](#游戏循环)
5. [关键算法](#关键算法)

---

## 整体架构

### 分层设计

```
┌─────────────────────────────────────────────────────────────┐
│                     表现层 (Presentation)                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   render    │  │    audio    │  │   snake2d_v2.rs     │  │
│  │  (渲染系统) │  │  (音效系统) │  │   (游戏主循环)      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                     逻辑层 (Logic)                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │    game     │  │   fruits    │  │     constants       │  │
│  │  (游戏逻辑) │  │  (果实系统) │  │     (配置常量)      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                     数据层 (Data)                            │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                       types                              ││
│  │  GameState | BuffState | Particle | Portal | Fruit | ... ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 依赖关系

```
snake2d_v2.rs (主入口)
    │
    ├── game (游戏逻辑)
    │   ├── snake.rs      → types, constants
    │   ├── collision.rs  → types, constants
    │   ├── spawn.rs      → types, constants, fruits
    │   ├── buff_manager.rs → types, constants
    │   ├── damage_manager.rs → types, constants
    │   ├── ai_snake.rs   → types, constants (AI蛇)
    │   └── ai_manager.rs → types, constants, fruits (AI蛇管理)
    │
    ├── render (渲染)
    │   ├── snake_renderer.rs → types, constants, game
    │   ├── fruit_renderer.rs → types, constants, fruits
    │   ├── effect_renderer.rs → types, constants
    │   ├── hud_renderer.rs → types, constants
    │   └── ai_snake_renderer.rs → types, constants, game (AI蛇渲染)
    │
    ├── audio (音效)
    │   └── sound_manager.rs → (无依赖)
    │
    ├── fruits (果实系统)
    │   ├── fruit_trait.rs → constants
    │   ├── fruit_registry.rs → fruit_trait
    │   └── [各种果实] → fruit_trait, types
    │       ├── normal/    (普通果实)
    │       ├── trap/      (陷阱果实)
    │       ├── power/     (功能果实)
    │       └── special/   (特殊果实: 幸运方块, 蛇蛋)
    │
    └── types (数据类型)
        └── (无依赖)
```

---

## 模块详解

### types 模块

定义所有游戏数据结构，是最底层的模块。

```rust
// 游戏状态枚举
pub enum GameState {
    Playing,   // 游戏进行中
    Paused,    // 暂停
    GameOver,  // 游戏结束
}

// Buff 状态（集中管理所有增益/减益）
pub struct BuffState {
    pub shield_active: bool,
    pub shield_timer: f32,
    // ... 其他 Buff
}

// 粒子（用于视觉特效）
pub struct Particle {
    pub pos: Vec2,      // 像素位置
    pub vel: Vec2,      // 速度
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
}
```

### game 模块

封装所有游戏逻辑，不包含任何渲染代码。

#### Snake 结构体

```rust
pub struct Snake {
    pub body: Vec<IVec2>,    // 身体位置，body[0] 是头
    pub dir: IVec2,          // 移动方向
    pub prev_body: Vec<IVec2>, // 上一帧位置（插值用）
}

impl Snake {
    // 移动算法
    pub fn move_forward(&mut self, wrap: bool, can_pass_self: bool) -> MoveResult {
        // 1. 保存上一帧位置
        self.prev_body = self.body.clone();
        
        // 2. 计算新头部位置
        let new_head = self.head() + self.dir;
        
        // 3. 处理边界
        if wrap { /* 穿墙 */ }
        else { /* 检查撞墙 */ }
        
        // 4. 检查自撞
        if !can_pass_self && self.body.contains(&new_head) {
            return MoveResult::SelfCollision;
        }
        
        // 5. 更新身体：插入头部，移除尾部
        self.body.insert(0, new_head);
        self.body.pop();
        
        MoveResult::Normal(new_head)
    }
}
```

#### 碰撞检测

```rust
// 检查是否碰到果实
pub fn check_fruit_collision(pos: IVec2, fruits: &[Fruit]) -> Option<usize> {
    fruits.iter().position(|f| f.pos == pos)
}

// 检查是否碰到传送门
pub fn check_portal_collision(pos: IVec2, portals: &[Portal]) -> Option<(IVec2, IVec2)> {
    for portal in portals {
        if pos == portal.pos_a {
            return Some((portal.pos_a, portal.pos_b));
        }
        // ...
    }
    None
}
```

### fruits 模块

基于 Trait 的可扩展果实系统。

```rust
// 核心 Trait
pub trait FruitBehavior: Send + Sync {
    fn config(&self) -> &FruitConfig;
    fn on_consume(&self, ctx: &mut FruitContext) -> ConsumeResult;
    fn render(&self, x: f32, y: f32, time: f32);
}

// 注册表
pub struct FruitRegistry {
    fruits: HashMap<&'static str, Box<dyn FruitBehavior>>,
    by_category: HashMap<FruitCategory, Vec<&'static str>>,
}

impl FruitRegistry {
    // 按类别随机选择（考虑权重和解锁条件）
    pub fn random_by_category(
        &self,
        category: FruitCategory,
        snake_length: usize,
        rng: &mut ThreadRng,
    ) -> Option<&'static str> {
        // 1. 过滤已解锁的果实
        // 2. 按权重随机选择
    }
}
```

### render 模块

纯渲染函数，不修改游戏状态。

```rust
// 蛇渲染（带各种视觉效果）
pub fn draw_snake(
    snake: &Snake,
    buff: &BuffState,
    damage_phase: DamagePhase,
    game_time: f32,
    wrap: bool,
    blend: f32,  // 插值比例
) {
    for (i, seg) in snake.body.iter().enumerate() {
        // 1. 根据 Buff 状态选择颜色
        let color = if buff.frozen { SKYBLUE }
                   else if buff.ghost_active { Color { a: 0.5, ..GREEN } }
                   // ...
        
        // 2. 计算插值位置
        let from = snake.prev_body.get(i).unwrap_or(seg);
        let x = lerp(from.x, seg.x, blend) * CELL;
        let y = lerp(from.y, seg.y, blend) * CELL;
        
        // 3. 绘制
        draw_rectangle(x, y, CELL, CELL, color);
    }
}
```

### audio 模块

程序化生成 WAV 音效。

```rust
// WAV 生成算法
pub fn make_tone_wav(freq: f32, dur: f32, vol: f32) -> Vec<u8> {
    let sr = 44100;  // 采样率
    let samples = (dur * sr as f32) as usize;
    
    // 生成正弦波采样
    let pcm: Vec<i16> = (0..samples)
        .map(|n| {
            let t = n as f32 / sr as f32;
            let s = (2.0 * PI * freq * t).sin();
            (s * vol * i16::MAX as f32) as i16
        })
        .collect();
    
    // 构建 WAV 文件
    let mut wav = Vec::new();
    wav.extend_from_slice(b"RIFF");
    // ... 写入头部和数据
    wav
}
```

---

## 数据流

### 输入处理

```
键盘输入 → 方向更新 → Snake.dir
         → 状态切换 → GameState
         → 模式切换 → wrap
```

### 游戏更新

```
accumulator += dt
while accumulator >= tick:
    Snake.move_forward()
        ↓
    MoveResult
        ├── Normal(new_head)
        │   ├── check_portal_collision() → teleport
        │   ├── check_food_collision() → grow + spawn_food
        │   └── check_fruit_collision() → handle_effect
        ├── WallCollision → GameOver (if !immune)
        └── SelfCollision → GameOver (if !immune)
```

### 渲染流程

```
clear_background()
    ↓
draw_border_and_grid()
    ↓
draw_blood_stains()  ← 最底层
    ↓
draw_portals()
    ↓
draw_food()
    ↓
draw_fruits()
    ↓
draw_afterimages()
    ↓
draw_shield_effect() / draw_ghost_effect()
    ↓
draw_snake()  ← 主体
    ↓
draw_particles()  ← 最顶层
    ↓
draw_overlay()  ← 覆盖层
    ↓
draw_hud()  ← HUD
```

---

## 游戏循环

### 固定时间步长

游戏使用固定时间步长（Fixed Timestep）确保逻辑一致性：

```rust
let mut accumulator = 0.0f32;

loop {
    // 计算 tick 间隔（根据蛇长度和 Buff 调整）
    let tick = calculate_tick(snake.len(), buff_state);
    
    // 累积帧时间
    accumulator += get_frame_time();
    
    // 固定步长更新
    while accumulator >= tick {
        accumulator -= tick;
        update_game_logic();  // 每次更新移动一格
    }
    
    // 渲染（使用插值）
    let blend = accumulator / tick;  // 0.0 ~ 1.0
    render(blend);
    
    next_frame().await;
}
```

### 插值渲染

为了平滑显示，渲染时在上一帧和当前帧位置之间插值：

```rust
// 计算插值位置
let from = prev_body[i];
let to = body[i];
let x = from.x + (to.x - from.x) * blend;
let y = from.y + (to.y - from.y) * blend;
```

---

## 关键算法

### 蛇移动

```
移动前: [H][1][2][T]
         ↓
计算新头: N = H + dir
         ↓
插入新头: [N][H][1][2][T]
         ↓
移除尾部: [N][H][1][2]
         ↓
移动后: [H'][1'][2'][T']
```

### 权重随机选择

```rust
// 假设有 3 个果实，权重分别为 10, 20, 30
// 总权重 = 60
// 随机数 roll = rand(0..60)

// roll = 15 时:
// 15 < 10? No, roll -= 10 → roll = 5
// 5 < 20? Yes → 选择第 2 个果实
```

### 受伤动画状态机

```
None → Flashing → Crumbling → None
         ↓           ↓
      闪烁效果    逐节移除尾巴
                 + 生成血液粒子
                 + 留下血迹
```

### 沙虫变身状态机

```
None → Flashing → Transforming → Exiting → Filling → FilledFlashing → Consuming → None
         ↓            ↓            ↓          ↓            ↓              ↓
      原地闪烁    逐节变色     移出边界   填满屏幕     闪烁变红       坍缩动画
```

### AI 蛇系统

AI 蛇是由蛇蛋果实触发生成的敌对蛇，增加游戏挑战性。

#### 数据结构

```rust
pub struct AISnake {
    pub body: Vec<IVec2>,      // 蛇身位置
    pub prev_body: Vec<IVec2>, // 上一帧位置（插值用）
    pub dir: IVec2,            // 移动方向
    pub color: Color,          // 随机颜色
    pub buff_state: BuffState, // 独立的 Buff 状态
    pub target: Option<IVec2>, // 当前目标位置
    pub think_timer: f32,      // 决策计时器
    pub id: u32,               // 唯一标识
    pub grow_pending: u32,     // 待增长节数
}

pub struct AIManager {
    pub snakes: Vec<AISnake>,       // 所有 AI 蛇
    pub max_snakes: usize,          // 最大数量（默认 3）
    pub dropped_foods: Vec<DroppedFood>, // 掉落的食物
}
```

#### AI 决策逻辑

```
每 0.2 秒执行一次决策:
    1. 寻找最近的目标（食物/果实）
    2. 计算期望方向
    3. 检查安全性（避开障碍）
    4. 更新移动方向
```

#### 碰撞规则

```
AI蛇头 → 玩家身体: AI蛇死亡，掉落食物
AI蛇头 → 其他AI蛇: AI蛇死亡，掉落食物
AI蛇头 → 自己身体: AI蛇死亡，掉落食物
AI蛇头 → 墙壁: AI蛇死亡，掉落食物
玩家蛇头 → AI蛇身体: 玩家死亡（除非有护盾/幽灵）
沙虫 → AI蛇: AI蛇被吞噬（不掉落）
```

#### 死亡掉落

```rust
// AI 蛇死亡时，每节身体变成一个食物
for pos in dead_snake.body {
    let offset = random(-1..=1, -1..=1);
    dropped_foods.push(DroppedFood {
        pos: pos + offset,
        spawn_time: game_time,
        lifetime: 10.0,  // 10秒后消失
    });
}
```

### 传送门穿越

传送门采用"逐节穿越"机制，类似穿墙效果：

```
蛇头进入入口 → 蛇头出现在出口 → 身体逐节跟随穿过
```

```rust
// 只传送蛇头，身体自然跟随
if let Some((from, to)) = check_portal_collision(new_head, &portals) {
    snake.body[0] = to;  // 只修改蛇头位置
}
```

---

## 性能考虑

### 内存管理

- 粒子系统自动回收过期粒子
- 果实和传送门超时自动移除
- 血迹有生命周期限制

### 渲染优化

- 使用简单的矩形绘制
- 避免每帧创建新对象
- 插值计算使用简单线性插值

### 逻辑优化

- 碰撞检测使用简单的位置比较
- 生成位置使用有限次重试
- Buff 更新使用简单的计时器递减
