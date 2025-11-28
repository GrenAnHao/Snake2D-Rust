#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
//! # Snake 2D - 增强版贪吃蛇游戏
//!
//! ## 游戏概述
//! 基于 Macroquad 引擎的 2D 贪吃蛇游戏，包含丰富的特殊果实系统和视觉效果。
//!
//! ## 核心特性
//! - **基础玩法**: 控制蛇吃果实增长，避免撞墙或撞到自己
//! - **特殊果实系统**: 可扩展的果实系统，支持多种果实类型
//! - **Buff系统**: 护盾、速度、幽灵等多种增益效果
//! - **视觉效果**: 粒子特效、残影、血迹、传送门动画等
//! - **Combo系统**: 连续吃果实获得额外分数
//!
//! ## 操作说明
//! - 方向键: 控制蛇的移动方向
//! - Space: 暂停/继续游戏
//! - Enter/R: 重新开始游戏
//! - W: 切换包围移动模式
//! - Esc/Q: 退出游戏

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
use ::rand::rngs::ThreadRng;

// ============================================================================
// 常量定义
// ============================================================================

/// 单元格像素尺寸（每个格子20x20像素）
const CELL: f32 = 20.0;
/// 游戏区域宽度（格子数）
const GRID_W: i32 = 32;
/// 游戏区域高度（格子数）
const GRID_H: i32 = 24;

// ============================================================================
// 枚举类型定义
// ============================================================================

/// 游戏状态枚举
/// 
/// 控制游戏的主循环行为：
/// - Playing: 正常游戏进行中，响应输入和更新逻辑
/// - Paused: 暂停状态，显示暂停覆盖层
/// - GameOver: 游戏结束，显示结束画面
#[derive(Clone, Copy, PartialEq)]
enum GameState { Playing, Paused, GameOver }

/// 果实类型枚举
/// 
/// 定义游戏中所有果实的类型，每种果实有不同的颜色和效果：
/// 
/// ## 普通果实
/// - Normal: 红色，吃到后蛇身增长1格
/// 
/// ## 陷阱果实（负面效果）
/// - Trap: 紫色，吃到后蛇身减少2格，触发受伤动画
/// - Freeze: 浅蓝色，吃到后冻住2秒无法移动
/// - Slow: 橙色，吃到后移动速度变慢5秒，蛇身抽搐
/// - Dizzy: 黄绿色，吃到后方向随机偏移5秒，醉酒效果
/// - Slime: 绿色，吃到后蛇身粘稠5秒，有拖拽抖动
/// 
/// ## 功能果实（正面效果，蛇长>=10时解锁）
/// - Shield: 金色，10秒无敌护盾
/// - Sandworm: 沙色，触发沙虫模式
/// - Speed: 蓝色，5秒移动速度加倍
/// - Ghost: 白色半透明，5秒可穿墙穿身
/// - Reverse: 粉色，蛇头尾互换
/// 
/// ## 特殊果实
/// - Lucky: 黄色礼盒带问号，随机触发增益或负面效果
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
enum FruitType {
    Normal,    // 红色，普通果实
    Trap,      // 紫色，陷阱果实
    Shield,    // 金色，护盾果实
    Sandworm,  // 沙色，沙虫果实
    Speed,     // 蓝色，速度果实
    Ghost,     // 白色半透明，幽灵果实
    Reverse,   // 粉色，逆转果实
    Freeze,    // 浅蓝色，冰冻陷阱
    Slow,      // 橙色，减速陷阱
    Dizzy,     // 黄绿色，眩晕果实
    Slime,     // 绿色，粘液果实
    Lucky,     // 黄色礼盒，幸运方块
}

impl FruitType {
    /// 获取果实对应的颜色
    fn color(&self) -> Color {
        match self {
            FruitType::Normal => RED,
            FruitType::Trap => Color { r: 0.6, g: 0.2, b: 0.8, a: 1.0 }, // 紫色
            FruitType::Shield => Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 }, // 金色
            FruitType::Sandworm => Color { r: 0.76, g: 0.60, b: 0.42, a: 1.0 }, // 沙色
            FruitType::Speed => Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 }, // 蓝色
            FruitType::Ghost => Color { r: 1.0, g: 1.0, b: 1.0, a: 0.7 }, // 白色半透明
            FruitType::Reverse => Color { r: 1.0, g: 0.4, b: 0.7, a: 1.0 }, // 粉色
            FruitType::Freeze => Color { r: 0.6, g: 0.9, b: 1.0, a: 1.0 }, // 浅蓝色
            FruitType::Slow => Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 }, // 橙色
            FruitType::Dizzy => Color { r: 0.8, g: 1.0, b: 0.3, a: 1.0 }, // 黄绿色
            FruitType::Slime => Color { r: 0.3, g: 0.8, b: 0.3, a: 1.0 }, // 绿色
            FruitType::Lucky => Color { r: 1.0, g: 0.9, b: 0.2, a: 1.0 }, // 黄色
        }
    }
    
    /// 是否是陷阱类型
    #[allow(dead_code)]
    fn is_trap(&self) -> bool {
        matches!(self, FruitType::Trap | FruitType::Freeze | FruitType::Slow | FruitType::Dizzy | FruitType::Slime)
    }
    
    /// 是否是功能果实（需要蛇长>=10才能生成）
    fn is_power(&self) -> bool {
        matches!(self, FruitType::Shield | FruitType::Sandworm | FruitType::Speed | FruitType::Ghost | FruitType::Reverse)
    }
}

// ============================================================================
// 数据结构定义
// ============================================================================

/// 果实结构体
/// 
/// 表示游戏中的一个果实实例，包含位置、类型和生命周期信息。
/// 
/// ## 字段说明
/// - `pos`: 果实在网格中的位置（整数坐标）
/// - `fruit_type`: 果实类型，决定颜色和效果
/// - `spawn_time`: 生成时的游戏时间戳，用于计算存在时长
/// - `lifetime`: 果实存在时长（秒），0表示永久存在
/// 
/// ## 生命周期
/// - 陷阱果实: 5秒后消失
/// - 功能果实: 3-8秒后消失（根据类型不同）
/// - 普通果实: 永久存在直到被吃掉
#[derive(Clone)]
struct Fruit {
    pos: IVec2,           // 网格位置
    fruit_type: FruitType, // 果实类型
    spawn_time: f32,      // 生成时间戳
    lifetime: f32,        // 存在时长（0=永久）
}

/// 粒子结构体
/// 
/// 用于实现各种视觉特效，如爆炸、血液飞溅等。
/// 
/// ## 物理模拟
/// - 粒子按照速度移动，并受到阻尼影响逐渐减速
/// - 生命周期结束后自动移除
/// 
/// ## 渲染
/// - 大小和透明度随生命周期衰减
/// - 颜色可自定义
#[derive(Clone)]
struct Particle {
    pos: Vec2,            // 当前位置（像素坐标）
    vel: Vec2,            // 速度向量
    color: Color,         // 粒子颜色
    lifetime: f32,        // 剩余生命周期
    max_lifetime: f32,    // 最大生命周期（用于计算衰减）
    size: f32,            // 粒子大小
}

/// 沙虫变身阶段枚举
/// 
/// 沙虫模式是一个复杂的多阶段动画序列：
/// 
/// ## 阶段流程
/// 1. **Flashing**: 蛇原地闪烁1.5秒，提示即将变身
/// 2. **Transforming**: 从头到尾逐节变成沙色
/// 3. **Exiting**: 沙虫向最近边界移动直到完全消失
/// 4. **Filling**: 从(0,0)开始顺时针填满整个游戏盘
/// 5. **FilledFlashing**: 填满后闪烁变红1.5秒
/// 6. **Consuming**: 所有格子向中心坍缩，产生爆炸效果
/// 
/// ## 胜利条件
/// 完成Consuming阶段后游戏胜利
#[derive(Clone, Copy, PartialEq)]
enum SandwormPhase {
    None,           // 未激活
    Flashing,       // 闪烁阶段（原地不动，身体闪烁）
    Transforming,   // 变色阶段（从头到尾变成沙色）
    Exiting,        // 退出阶段（蛇向边界移动直到消失）
    Filling,        // 填充阶段（蛇不断增长填满整个游戏盘）
    FilledFlashing, // 填满后闪烁变红
    Consuming,      // 吞噬阶段（坍缩动画）
}

/// 受伤状态结构体
/// 
/// 管理吃到陷阱果实后的受伤动画，包括红色闪烁和尾巴碎裂。
/// 
/// ## 动画流程
/// 1. **Flashing阶段**: 蛇身快速闪烁红色（类似Minecraft受伤效果）
/// 2. **Crumbling阶段**: 尾巴一节一节碎裂，产生红色粒子和血迹
/// 
/// ## 触发条件
/// - 吃到Trap果实且没有护盾/幽灵/沙虫模式保护
/// - 蛇长度必须>3，否则直接游戏结束
#[derive(Clone)]
struct DamageState {
    active: bool,              // 是否正在播放受伤动画
    flash_timer: f32,          // 当前闪烁周期计时器
    flash_count: i32,          // 已完成的闪烁次数
    crumble_timer: f32,        // 碎裂间隔计时器
    crumble_index: usize,      // 当前碎裂到第几节尾巴
    tail_to_remove: Vec<IVec2>, // 待移除的尾巴位置列表
    phase: DamagePhase,        // 当前动画阶段
}

/// 受伤动画阶段枚举
#[derive(Clone, Copy, PartialEq)]
enum DamagePhase {
    None,       // 无受伤动画
    Flashing,   // 红色闪烁阶段（约0.6秒，闪烁4次）
    Crumbling,  // 尾巴碎裂阶段（每0.15秒碎一节）
}

impl Default for DamageState {
    fn default() -> Self {
        DamageState {
            active: false,
            flash_timer: 0.0,
            flash_count: 0,
            crumble_timer: 0.0,
            crumble_index: 0,
            tail_to_remove: vec![],
            phase: DamagePhase::None,
        }
    }
}

/// 血迹结构体
/// 
/// 当蛇尾碎裂时在地面留下的血迹，会随时间淡出消失。
/// 
/// ## 视觉效果
/// - 主血迹：暗红色方块
/// - 溅射：周围随机分布的小血滴
/// - 淡出：5秒内逐渐透明直到消失
#[derive(Clone)]
struct BloodStain {
    pos: IVec2,       // 网格位置
    spawn_time: f32,  // 生成时间
    lifetime: f32,    // 存在时长（默认5秒）
    size: f32,        // 大小系数（0.6-1.0）
    alpha: f32,       // 初始透明度（0.5-0.8）
}

/// 传送门结构体
/// 
/// 成对出现的传送门，蛇进入一个会从另一个出来。
/// 
/// ## 生成规则
/// - 每15秒有30%概率生成
/// - 两个传送门位置不重叠
/// - 存在20秒后消失
/// 
/// ## 视觉效果
/// - 彩色脉动光环
/// - 两个传送门之间有连接线
/// - 最后3秒开始闪烁提示即将消失
#[derive(Clone)]
struct Portal {
    pos_a: IVec2,     // 传送门A位置
    pos_b: IVec2,     // 传送门B位置
    color: Color,     // 传送门颜色
    spawn_time: f32,  // 生成时间
    lifetime: f32,    // 存在时长
}

/// Combo系统结构体
/// 
/// 追踪连续吃果实的连击数，提供额外分数奖励。
/// 
/// ## 连击规则
/// - 1.5秒内连续吃果实会累积combo
/// - combo数越高，每个果实额外加分越多（最高+5）
/// - 吃到陷阱果实会重置combo
/// 
/// ## 显示
/// - combo>1时在屏幕右上角显示"COMBO x数字!"
/// - 颜色随combo数变化：黄->橙->红
#[derive(Clone)]
struct ComboState {
    count: u32,           // 当前连击数
    #[allow(dead_code)]
    timer: f32,           // 连击计时器（保留字段）
    last_eat_time: f32,   // 上次吃果实的游戏时间
    display_timer: f32,   // combo显示剩余时间
}

impl Default for ComboState {
    fn default() -> Self {
        ComboState {
            count: 0,
            timer: 0.0,
            last_eat_time: -10.0, // 初始化为负数，确保第一次吃果实不算连击
            display_timer: 0.0,
        }
    }
}

/// 残影结构体
/// 
/// 速度模式下蛇移动时留下的蓝色残影效果。
/// 
/// ## 实现原理
/// - 每次移动时记录当前蛇身位置
/// - 残影随时间淡出（0.3秒）
/// - 颜色为半透明蓝色
#[derive(Clone)]
struct Afterimage {
    positions: Vec<IVec2>, // 残影的蛇身位置快照
    alpha: f32,
    spawn_time: f32,
}

/// Buff状态管理结构体
/// 
/// 集中管理所有增益/减益效果的状态，包括：
/// 
/// ## 护盾模式 (Shield)
/// - 持续10秒
/// - 免疫所有陷阱效果
/// - 显示金色光环
/// 
/// ## 沙虫模式 (Sandworm)
/// - 触发复杂的多阶段变身动画
/// - 最终填满整个游戏盘并胜利
/// 
/// ## 速度模式 (Speed)
/// - 持续5秒
/// - 移动速度加倍
/// - 蛇身变蓝色，留下残影
/// 
/// ## 幽灵模式 (Ghost)
/// - 持续5秒
/// - 可以穿墙、穿过自己
/// - 免疫陷阱效果
/// - 蛇身半透明
/// 
/// ## 冰冻状态 (Frozen)
/// - 持续2秒
/// - 无法移动和改变方向
/// - 蛇身变浅蓝色
#[derive(Clone)]
struct BuffState {
    // === 护盾模式 ===
    shield_active: bool,         // 护盾是否激活
    shield_timer: f32,           // 护盾剩余时间
    
    // === 沙虫模式 ===
    sandworm_active: bool,       // 沙虫模式是否激活
    sandworm_path: Vec<IVec2>,   // 顺时针填充路径
    sandworm_index: usize,       // 当前路径索引
    sandworm_tick: f32,          // 移动计时器
    
    // 沙虫变身动画状态
    sandworm_phase: SandwormPhase,       // 当前变身阶段
    sandworm_phase_timer: f32,           // 阶段计时器
    sandworm_transform_index: usize,     // 变色进度（已变色节数）
    sandworm_original_snake: Vec<IVec2>, // 变身前蛇身位置
    sandworm_original_dir: IVec2,        // 变身前方向
    
    // 坍缩动画状态
    sandworm_collapse_center: Vec2,      // 坍缩中心点
    sandworm_collapse_progress: f32,     // 坍缩进度 0.0->1.0
    sandworm_collapse_positions: Vec<Vec2>, // 各格子动画位置
    
    // 退出阶段状态
    sandworm_exit_dir: IVec2,    // 退出方向
    
    // === 速度模式 ===
    speed_active: bool,          // 速度模式是否激活
    speed_timer: f32,            // 速度模式剩余时间
    
    // === 幽灵模式 ===
    ghost_active: bool,          // 幽灵模式是否激活
    ghost_timer: f32,            // 幽灵模式剩余时间
    
    // === 冰冻状态 ===
    frozen: bool,                // 是否被冰冻
    freeze_timer: f32,           // 冰冻剩余时间
    
    // === 减速状态 ===
    slow_active: bool,           // 是否减速中
    slow_timer: f32,             // 减速剩余时间
    
    // === 眩晕状态 ===
    dizzy_active: bool,          // 是否眩晕中
    dizzy_timer: f32,            // 眩晕剩余时间
    
    // === 粘液状态 ===
    slime_active: bool,          // 是否粘液中
    slime_timer: f32,            // 粘液剩余时间
}

impl Default for BuffState {
    fn default() -> Self {
        BuffState {
            shield_active: false,
            shield_timer: 0.0,
            sandworm_active: false,
            sandworm_path: vec![],
            sandworm_index: 0,
            sandworm_tick: 0.0,
            sandworm_phase: SandwormPhase::None,
            sandworm_phase_timer: 0.0,
            sandworm_transform_index: 0,
            sandworm_original_snake: vec![],
            sandworm_original_dir: ivec2(1, 0),
            sandworm_collapse_center: vec2(0.0, 0.0),
            sandworm_collapse_progress: 0.0,
            sandworm_collapse_positions: vec![],
            sandworm_exit_dir: ivec2(1, 0),
            speed_active: false,
            speed_timer: 0.0,
            ghost_active: false,
            ghost_timer: 0.0,
            frozen: false,
            freeze_timer: 0.0,
            slow_active: false,
            slow_timer: 0.0,
            dizzy_active: false,
            dizzy_timer: 0.0,
            slime_active: false,
            slime_timer: 0.0,
        }
    }
}

// ============================================================================
// 窗口配置
// ============================================================================

/// 窗口配置函数
/// 
/// 设置游戏窗口的基本属性：
/// - 窗口大小：640x480像素（32x24格子，每格20像素）
/// - 禁止调整大小：保持像素完美
/// - 禁用高DPI：避免缩放问题
fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake 2D"),
        window_width: (GRID_W as f32 * CELL) as i32,  // 640像素
        window_height: (GRID_H as f32 * CELL) as i32, // 480像素
        fullscreen: false,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

// ============================================================================
// 主函数 - 游戏入口点
// ============================================================================

/// 游戏主函数
/// 
/// ## 游戏循环结构
/// 1. **初始化**: 创建蛇、食物、音效等
/// 2. **输入处理**: 方向键、暂停、重启等
/// 3. **游戏逻辑更新**: 移动、碰撞检测、Buff更新
/// 4. **渲染**: 绘制所有游戏元素
/// 5. **循环**: 等待下一帧
/// 
/// ## 时间步进
/// 使用固定时间步进（tick）确保游戏速度稳定：
/// - 基础tick: 120ms
/// - 随蛇长加速: 每3节减少5ms，最快40ms
/// - 速度模式: tick减半
#[macroquad::main(window_conf)]
async fn main() {
    // ========== 蛇的状态 ==========
    // 初始蛇身：3节，位于屏幕中央，水平向右
    let mut snake: Vec<IVec2> = vec![
        ivec2(GRID_W / 2, GRID_H / 2),     // 头部
        ivec2(GRID_W / 2 - 1, GRID_H / 2), // 身体
        ivec2(GRID_W / 2 - 2, GRID_H / 2), // 尾部
    ];
    let mut prev_snake: Vec<IVec2> = snake.clone(); // 上一帧位置（用于插值渲染）
    let mut dir: IVec2 = ivec2(1, 0); // 移动方向（右）
    
    // ========== 随机数和食物 ==========
    let mut rng = thread_rng();
    let mut food = spawn_food(&snake, &mut rng);
    
    // ========== 分数系统 ==========
    let mut score: u32 = 0;
    let mut high_score: u32 = 0; // 本次会话最高分
    
    // ========== 游戏状态 ==========
    let mut state = GameState::Playing;
    let mut wrap = true; // 包围移动模式（穿墙）
    
    // ========== 音效 ==========
    // 预生成简单的正弦波音效
    let eat_snd: Sound = load_sound_from_bytes(&make_tone_wav(880.0, 0.08, 0.4)).await.unwrap();
    let over_snd: Sound = load_sound_from_bytes(&make_tone_wav(220.0, 0.30, 0.5)).await.unwrap();
    let trap_snd: Sound = load_sound_from_bytes(&make_tone_wav(330.0, 0.15, 0.5)).await.unwrap();
    let power_snd: Sound = load_sound_from_bytes(&make_tone_wav(660.0, 0.2, 0.6)).await.unwrap();
    let mut over_once = false;

    // 特殊果实系统状态
    let mut fruits: Vec<Fruit> = vec![];
    let mut particles: Vec<Particle> = vec![];
    let mut buff_state = BuffState::default();
    let mut power_fruit_enabled = false;
    let mut game_time: f32 = 0.0;
    let mut trap_spawn_timer: f32 = 0.0;
    
    // 受伤状态和血迹
    let mut damage_state = DamageState::default();
    let mut blood_stains: Vec<BloodStain> = vec![];
    
    // 传送门、Combo、残影
    let mut portals: Vec<Portal> = vec![];
    let mut combo_state = ComboState::default();
    let mut afterimages: Vec<Afterimage> = vec![];
    let mut portal_spawn_timer: f32 = 0.0;

    // 时间累加器：按 tick 固定步进，帧率波动时保持游戏速度稳定
    let mut accumulator = 0.0f32;

    loop {
        // 随长度加速：随蛇长增加逐步缩短 tick，40ms~120ms 区间
        let len = snake.len() as u32;
        let mut tick = ((120u32.saturating_sub((len / 3) * 5)).max(40)) as f32 / 1000.0;
        // 速度模式：tick减半（移动更快）
        if buff_state.speed_active {
            tick *= 0.5;
        }
        // 减速模式：tick加倍（移动更慢）
        if buff_state.slow_active {
            tick *= 2.0;
        }
        // 粘液模式：tick增加50%
        if buff_state.slime_active {
            tick *= 1.5;
        }
        accumulator += get_frame_time();

        // 基础输入：退出、暂停/恢复、重开、包围切换
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) { break; }
        if is_key_pressed(KeyCode::Space) {
            state = match state { GameState::Playing => GameState::Paused, GameState::Paused => GameState::Playing, s => s };
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R) {
            if state != GameState::Playing {
                restart(&mut snake, &mut dir, &mut rng, &mut food, &mut score);
                prev_snake = snake.clone();
                state = GameState::Playing;
                over_once = false;
                // 重置特殊果实状态
                fruits.clear();
                particles.clear();
                buff_state = BuffState::default();
                power_fruit_enabled = false;
                game_time = 0.0;
                trap_spawn_timer = 0.0;
                // 重置受伤状态和血迹
                damage_state = DamageState::default();
                blood_stains.clear();
                // 重置传送门、Combo、残影
                portals.clear();
                combo_state = ComboState::default();
                afterimages.clear();
                portal_spawn_timer = 0.0;
            }
        }
        if is_key_pressed(KeyCode::W) { wrap = !wrap; }
        // 方向输入仅在进行中有效，且禁止反向瞬间掉头，冰冻时无法移动
        if state == GameState::Playing && !buff_state.frozen {
            let mut new_dir = dir;
            if is_key_pressed(KeyCode::Up) && dir.y != 1 { new_dir = ivec2(0, -1); }
            if is_key_pressed(KeyCode::Down) && dir.y != -1 { new_dir = ivec2(0, 1); }
            if is_key_pressed(KeyCode::Left) && dir.x != 1 { new_dir = ivec2(-1, 0); }
            if is_key_pressed(KeyCode::Right) && dir.x != -1 { new_dir = ivec2(1, 0); }
            
            // 眩晕效果：有概率随机偏移方向
            if buff_state.dizzy_active && new_dir != dir {
                if rng.gen_bool(0.4) { // 40%概率方向偏移
                    let directions = [ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)];
                    let valid_dirs: Vec<_> = directions.iter()
                        .filter(|&&d| d != ivec2(-dir.x, -dir.y)) // 排除反向
                        .collect();
                    if !valid_dirs.is_empty() {
                        new_dir = *valid_dirs[rng.gen_range(0..valid_dirs.len())];
                    }
                }
            }
            dir = new_dir;
        }

        // 固定步进：每 tick 推进一次，限制每帧最多步进 5 次防止卡顿突增
        let mut steps = 0;
        while accumulator >= tick && steps < 5 {
            accumulator -= tick;
            if state != GameState::Playing { break; }
            // 冰冻时跳过移动
            if buff_state.frozen { continue; }
            steps += 1;

            prev_snake = snake.clone();
            
            // 速度模式：记录残影
            if buff_state.speed_active {
                afterimages.push(Afterimage {
                    positions: snake.clone(),
                    alpha: 0.4,
                    spawn_time: game_time,
                });
            }

            // 计算下一头部坐标
            let head = snake[0];
            let mut nx = head.x + dir.x;
            let mut ny = head.y + dir.y;
            
            // 幽灵模式：可以穿墙
            if buff_state.ghost_active {
                if nx < 0 { nx = GRID_W - 1; }
                if nx >= GRID_W { nx = 0; }
                if ny < 0 { ny = GRID_H - 1; }
                if ny >= GRID_H { ny = 0; }
            } else if wrap {
                // 包围移动：越界从另一侧出现
                if nx < 0 { nx = GRID_W - 1; }
                if nx >= GRID_W { nx = 0; }
                if ny < 0 { ny = GRID_H - 1; }
                if ny >= GRID_H { ny = 0; }
            }
            let mut new_head = ivec2(nx, ny);
            
            // 检查传送门：蛇头进入传送门时，整条蛇从另一端出来
            let mut teleported = false;
            for portal in &portals {
                let (from, to) = if new_head == portal.pos_a {
                    (portal.pos_a, portal.pos_b)
                } else if new_head == portal.pos_b {
                    (portal.pos_b, portal.pos_a)
                } else {
                    continue;
                };
                
                // 计算传送偏移量
                let offset = ivec2(to.x - from.x, to.y - from.y);
                
                // 传送整条蛇（所有节点都加上偏移量）
                for seg in snake.iter_mut() {
                    seg.x += offset.x;
                    seg.y += offset.y;
                    // 处理边界包围
                    if seg.x < 0 { seg.x += GRID_W; }
                    if seg.x >= GRID_W { seg.x -= GRID_W; }
                    if seg.y < 0 { seg.y += GRID_H; }
                    if seg.y >= GRID_H { seg.y -= GRID_H; }
                }
                new_head = ivec2(new_head.x + offset.x, new_head.y + offset.y);
                // 处理新头部边界
                if new_head.x < 0 { new_head.x += GRID_W; }
                if new_head.x >= GRID_W { new_head.x -= GRID_W; }
                if new_head.y < 0 { new_head.y += GRID_H; }
                if new_head.y >= GRID_H { new_head.y -= GRID_H; }
                
                teleported = true;
                break;
            }
            let _ = teleported; // 避免未使用警告

            // 非包围模式下，越界即结束（幽灵模式除外）
            if !wrap && !buff_state.ghost_active {
                if new_head.x < 0 || new_head.x >= GRID_W || new_head.y < 0 || new_head.y >= GRID_H {
                    state = GameState::GameOver;
                }
            }

            // 沙虫模式的所有阶段都跳过正常移动逻辑
            if buff_state.sandworm_phase != SandwormPhase::None {
                continue;
            }
            
            // 自撞判定（护盾模式、沙虫模式或幽灵模式下可以穿过自己）
            if state == GameState::GameOver { break; }
            if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active && snake.iter().any(|&p| p == new_head) {
                state = GameState::GameOver;
                break;
            }

            // 推进：插入新头；吃到食物则加分并生成新食物，否则移除尾部
            snake.insert(0, new_head);
            let ate = new_head == food;
            if ate {
                score += 1;
                food = spawn_food(&snake, &mut rng);
                play_sound(&eat_snd, PlaySoundParams { looped: false, volume: 0.5 });
            } else {
                snake.pop();
            }
            
            // 检查特殊果实碰撞
            let mut consumed_idx = None;
            for (i, fruit) in fruits.iter().enumerate() {
                if fruit.pos == new_head {
                    consumed_idx = Some(i);
                    break;
                }
            }
            
            if let Some(idx) = consumed_idx {
                let fruit = fruits.remove(idx);
                // 更新Combo
                update_combo(&mut combo_state, game_time);
                
                match fruit.fruit_type {
                    FruitType::Normal => {
                        let combo_bonus = combo_state.count.min(5);
                        score += 1 + combo_bonus;
                        play_sound(&eat_snd, PlaySoundParams { looped: false, volume: 0.5 });
                    }
                    FruitType::Trap => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 如果有护盾、沙虫模式或幽灵模式，免疫
                        if buff_state.shield_active || buff_state.sandworm_active || buff_state.ghost_active {
                            // 免疫，不做任何事
                        } else if snake.len() <= 3 {
                            // 蛇太短，直接死亡
                            state = GameState::GameOver;
                        } else {
                            // 触发受伤动画
                            start_damage_animation(&mut damage_state, &snake);
                        }
                        combo_state.count = 0; // 吃到陷阱重置combo
                    }
                    FruitType::Shield => {
                        play_sound(&power_snd, PlaySoundParams { looped: false, volume: 0.7 });
                        activate_shield(&mut buff_state);
                    }
                    FruitType::Sandworm => {
                        play_sound(&power_snd, PlaySoundParams { looped: false, volume: 0.7 });
                        activate_sandworm(&mut buff_state, &snake, dir);
                    }
                    FruitType::Speed => {
                        play_sound(&power_snd, PlaySoundParams { looped: false, volume: 0.7 });
                        buff_state.speed_active = true;
                        buff_state.speed_timer = 5.0; // 5秒速度模式
                    }
                    FruitType::Ghost => {
                        play_sound(&power_snd, PlaySoundParams { looped: false, volume: 0.7 });
                        buff_state.ghost_active = true;
                        buff_state.ghost_timer = 5.0; // 5秒幽灵模式
                    }
                    FruitType::Reverse => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 逆转蛇身：头尾互换
                        snake.reverse();
                        // 更新方向为新头部的方向
                        if snake.len() > 1 {
                            let new_head = snake[0];
                            let second = snake[1];
                            dir = ivec2(new_head.x - second.x, new_head.y - second.y);
                        }
                    }
                    FruitType::Freeze => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 如果有护盾、沙虫模式或幽灵模式，免疫
                        if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active {
                            buff_state.frozen = true;
                            buff_state.freeze_timer = 2.0; // 冰冻2秒
                            // 生成初始寒气粒子爆发
                            spawn_freeze_particles(&mut particles, &snake, &mut rng);
                        }
                        combo_state.count = 0; // 吃到陷阱重置combo
                    }
                    FruitType::Slow => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 如果有护盾、沙虫模式或幽灵模式，免疫
                        if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active {
                            buff_state.slow_active = true;
                            buff_state.slow_timer = 5.0; // 减速5秒
                        }
                        combo_state.count = 0;
                    }
                    FruitType::Dizzy => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 如果有护盾、沙虫模式或幽灵模式，免疫
                        if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active {
                            buff_state.dizzy_active = true;
                            buff_state.dizzy_timer = 5.0; // 眩晕5秒
                        }
                        combo_state.count = 0;
                    }
                    FruitType::Slime => {
                        play_sound(&trap_snd, PlaySoundParams { looped: false, volume: 0.6 });
                        // 如果有护盾、沙虫模式或幽灵模式，免疫
                        if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active {
                            buff_state.slime_active = true;
                            buff_state.slime_timer = 5.0; // 粘液5秒
                        }
                        combo_state.count = 0;
                    }
                    FruitType::Lucky => {
                        // 幸运方块：随机触发增益或负面效果
                        play_sound(&power_snd, PlaySoundParams { looped: false, volume: 0.7 });
                        // 生成开箱粒子效果
                        let center = vec2(
                            fruit.pos.x as f32 * CELL + CELL / 2.0,
                            fruit.pos.y as f32 * CELL + CELL / 2.0
                        );
                        spawn_lucky_particles(&mut particles, center, &mut rng);
                        
                        // 50%增益，50%负面
                        let is_positive = rng.gen_bool(0.5);
                        if is_positive {
                            // 增益效果（随机选一个）
                            match rng.gen_range(0..5) {
                                0 => {
                                    // 护盾
                                    buff_state.shield_active = true;
                                    buff_state.shield_timer = 10.0;
                                }
                                1 => {
                                    // 速度
                                    buff_state.speed_active = true;
                                    buff_state.speed_timer = 5.0;
                                }
                                2 => {
                                    // 幽灵
                                    buff_state.ghost_active = true;
                                    buff_state.ghost_timer = 5.0;
                                }
                                3 => {
                                    // 额外分数
                                    score += 10;
                                }
                                _ => {
                                    // 蛇身增长3节
                                    for _ in 0..3 {
                                        if let Some(tail) = snake.last().cloned() {
                                            snake.push(tail);
                                        }
                                    }
                                }
                            }
                        } else {
                            // 负面效果（随机选一个，受免疫影响）
                            if !buff_state.shield_active && !buff_state.sandworm_active && !buff_state.ghost_active {
                                match rng.gen_range(0..5) {
                                    0 => {
                                        // 冰冻
                                        buff_state.frozen = true;
                                        buff_state.freeze_timer = 2.0;
                                        spawn_freeze_particles(&mut particles, &snake, &mut rng);
                                    }
                                    1 => {
                                        // 减速
                                        buff_state.slow_active = true;
                                        buff_state.slow_timer = 5.0;
                                    }
                                    2 => {
                                        // 眩晕
                                        buff_state.dizzy_active = true;
                                        buff_state.dizzy_timer = 5.0;
                                    }
                                    3 => {
                                        // 粘液
                                        buff_state.slime_active = true;
                                        buff_state.slime_timer = 5.0;
                                    }
                                    _ => {
                                        // 蛇身减少（如果够长）
                                        if snake.len() > 3 {
                                            start_damage_animation(&mut damage_state, &snake);
                                        }
                                    }
                                }
                            }
                            combo_state.count = 0;
                        }
                    }
                }
            }
        }

        // 更新游戏时间和特殊系统
        let dt = get_frame_time();
        if state == GameState::Playing {
            game_time += dt;
            
            // 更新护盾计时器
            update_shield(&mut buff_state, dt);
            
            // 更新速度模式
            if buff_state.speed_active {
                buff_state.speed_timer -= dt;
                if buff_state.speed_timer <= 0.0 {
                    buff_state.speed_active = false;
                }
            }
            
            // 更新幽灵模式
            if buff_state.ghost_active {
                buff_state.ghost_timer -= dt;
                if buff_state.ghost_timer <= 0.0 {
                    buff_state.ghost_active = false;
                }
            }
            
            // 更新冰冻状态
            if buff_state.frozen {
                buff_state.freeze_timer -= dt;
                // 持续生成寒气粒子
                if rng.gen_bool(0.3) { // 30%概率每帧生成
                    spawn_frost_ambient_particles(&mut particles, &snake, &mut rng);
                }
                if buff_state.freeze_timer <= 0.0 {
                    buff_state.frozen = false;
                }
            }
            
            // 更新减速状态
            if buff_state.slow_active {
                buff_state.slow_timer -= dt;
                if buff_state.slow_timer <= 0.0 {
                    buff_state.slow_active = false;
                }
            }
            
            // 更新眩晕状态
            if buff_state.dizzy_active {
                buff_state.dizzy_timer -= dt;
                if buff_state.dizzy_timer <= 0.0 {
                    buff_state.dizzy_active = false;
                }
            }
            
            // 更新粘液状态
            if buff_state.slime_active {
                buff_state.slime_timer -= dt;
                if buff_state.slime_timer <= 0.0 {
                    buff_state.slime_active = false;
                }
            }
            
            // 更新沙虫模式
            if buff_state.sandworm_active {
                // 保存坍缩中心用于爆炸粒子
                let collapse_center = buff_state.sandworm_collapse_center;
                
                // 处理所有沙虫阶段
                let (_, win) = update_sandworm_transform(&mut buff_state, &mut snake, dt);
                
                // 在填充阶段，吞噬头部位置的所有果实（仅移除，无效果）
                if buff_state.sandworm_phase == SandwormPhase::Filling {
                    if let Some(head) = snake.first() {
                        // 吞噬特殊果实
                        fruits.retain(|f| f.pos != *head);
                        // 吞噬普通果实（移到屏幕外，不重新生成，避免填满时无限循环）
                        if *head == food {
                            food = ivec2(-1, -1); // 移到屏幕外
                        }
                    }
                }
                
                if win {
                    // 沙虫模式完成，生成鲜血喷溅爆炸粒子
                    spawn_blood_explosion(&mut particles, collapse_center, &mut rng);
                    // 游戏结束（沙虫吞噬完毕）
                    state = GameState::GameOver;
                }
            }
            
            
            // 更新果实过期
            let _ = update_fruits(&mut fruits, game_time);
            
            // 陷阱果实生成（非沙虫模式时）
            if !buff_state.sandworm_active {
                trap_spawn_timer += dt;
                if trap_spawn_timer >= 3.0 {
                    trap_spawn_timer = 0.0;
                    if rng.gen_bool(0.4) {
                        if let Some(trap) = spawn_trap_fruit(&snake, &fruits, game_time, &mut rng) {
                            fruits.push(trap);
                        }
                    }
                }
            }
            
            // 功能果实解锁和生成
            if !power_fruit_enabled && should_enable_power_fruit(snake.len()) {
                power_fruit_enabled = true;
            }
            if power_fruit_enabled && !fruits.iter().any(|f| f.fruit_type.is_power()) {
                if rng.gen_bool(0.015) {
                    if let Some(power) = spawn_power_fruit(&snake, &fruits, game_time, &mut rng) {
                        fruits.push(power);
                    }
                }
            }
            
            // 幸运方块生成（每5秒有20%概率生成，场上最多1个）
            if !buff_state.sandworm_active && !fruits.iter().any(|f| f.fruit_type == FruitType::Lucky) {
                if rng.gen_bool(0.003) { // 约每5秒检查一次
                    if let Some(lucky) = spawn_lucky_fruit(&snake, &fruits, game_time, &mut rng) {
                        fruits.push(lucky);
                    }
                }
            }
            
            // 传送门生成
            portal_spawn_timer += dt;
            if portal_spawn_timer >= 15.0 && portals.is_empty() {
                portal_spawn_timer = 0.0;
                if rng.gen_bool(0.3) {
                    if let Some(portal) = spawn_portal(&snake, &fruits, game_time, &mut rng) {
                        portals.push(portal);
                    }
                }
            }
            
            // 更新传送门
            update_portals(&mut portals, game_time);
            
            // 更新残影
            update_afterimages(&mut afterimages, game_time);
            
            // 更新Combo显示
            combo_state.display_timer -= dt;
        }
        
        // 更新粒子
        update_particles(&mut particles, dt);
        
        // 更新受伤动画
        if damage_state.active {
            update_damage_animation(&mut damage_state, &mut snake, &mut particles, &mut blood_stains, game_time, dt, &mut rng);
        }
        
        // 更新血迹（淡出消失）
        update_blood_stains(&mut blood_stains, game_time);

        // 渲染：背景 -> 边框+网格 -> 血迹 -> 食物 -> 蛇 -> 覆盖层（如有） -> HUD 文本
        clear_background(DARKGRAY);
        draw_border_and_grid();
        
        // 绘制血迹（在食物和蛇之前）
        draw_blood_stains(&blood_stains, game_time);
        
        // 绘制传送门
        draw_portals(&portals, game_time);
        
        // 绘制普通食物
        draw_rectangle(food.x as f32 * CELL, food.y as f32 * CELL, CELL, CELL, RED);
        
        // 绘制特殊果实
        for fruit in &fruits {
            let x = fruit.pos.x as f32 * CELL;
            let y = fruit.pos.y as f32 * CELL;
            
            if fruit.fruit_type == FruitType::Sandworm {
                // 沙虫果实：使用斑驳纹理
                draw_sandworm_fruit(x, y, game_time, fruit.pos.x, fruit.pos.y);
            } else if fruit.fruit_type == FruitType::Lucky {
                // 幸运方块：黄色礼盒带问号
                draw_lucky_fruit(x, y, game_time);
            } else {
                let color = fruit.fruit_type.color();
                draw_rectangle(x, y, CELL, CELL, color);
            }
            // 有时限的果实显示进度条（陷阱果实和功能果实）
            if fruit.lifetime > 0.0 {
                let progress = calc_progress(fruit.spawn_time, fruit.lifetime, game_time);
                draw_progress_bar(fruit.pos, progress);
            }
        }
        

        let mut blend = if state == GameState::Playing {
            accumulator / tick
        } else {
            0.0
        };
        if blend < 0.0 { blend = 0.0; }
        if blend > 1.0 { blend = 1.0; }

        // 绘制残影（速度模式）
        draw_afterimages(&afterimages, game_time);
        
        // 绘制护盾效果（在蛇之前）
        if buff_state.shield_active {
            draw_shield_effect(&snake, game_time);
        }
        
        // 绘制幽灵效果（在蛇之前）
        if buff_state.ghost_active {
            draw_ghost_effect(&snake, game_time);
        }
        
        // 绘制蛇身（根据沙虫变身状态使用不同渲染）
        if buff_state.sandworm_phase == SandwormPhase::Flashing || buff_state.sandworm_phase == SandwormPhase::Transforming {
            // 变身动画：使用原始蛇身位置
            draw_sandworm_transform(&buff_state.sandworm_original_snake, &buff_state, game_time);
        } else if buff_state.sandworm_phase == SandwormPhase::Exiting {
            // 退出阶段：沙虫向边界移动
            for (i, seg) in snake.iter().enumerate() {
                // 只绘制在屏幕内的部分
                if seg.x >= 0 && seg.x < GRID_W && seg.y >= 0 && seg.y < GRID_H {
                    let x = seg.x as f32 * CELL;
                    let y = seg.y as f32 * CELL;
                    if i == 0 {
                        draw_sandworm_head(x, y, buff_state.sandworm_exit_dir);
                    } else {
                        draw_sandworm_segment(x, y, CELL, sandworm_body_color(), seg.x, seg.y, game_time);
                    }
                }
            }
        } else if buff_state.sandworm_phase == SandwormPhase::Filling {
            // 填充阶段：沙色身体 + 特殊头部 + 蠕动效果
            let snake_len = snake.len() as f32;
            
            for (i, seg) in snake.iter().enumerate() {
                let base_x = seg.x as f32 * CELL;
                let base_y = seg.y as f32 * CELL;
                let idx = i as f32;
                
                // 蠕动效果：波浪式位移
                let wave_speed = 8.0;
                let wave_amplitude = 2.0;
                let wave_offset = (game_time * wave_speed + idx * 0.3).sin() * wave_amplitude;
                
                // 大小脉动
                let pulse_speed = 6.0;
                let pulse_amplitude = 0.1;
                let size_pulse = 1.0 + (game_time * pulse_speed + idx * 0.2).sin() * pulse_amplitude;
                let size = CELL * size_pulse;
                let size_offset = (CELL - size) / 2.0;
                
                // 颜色渐变：从头到尾逐渐变浅
                let color_factor = 1.0 - (idx / snake_len.max(1.0)) * 0.15;
                let body_color = sandworm_body_color();
                let segment_color = Color {
                    r: body_color.r * color_factor,
                    g: body_color.g * color_factor,
                    b: body_color.b * color_factor,
                    a: 1.0,
                };
                
                if i == 0 {
                    // 头部：计算方向
                    let head_dir = if snake.len() > 1 {
                        let next = snake[1];
                        ivec2(seg.x - next.x, seg.y - next.y)
                    } else {
                        ivec2(1, 0)
                    };
                    // 头部也有轻微蠕动
                    let head_wave = (game_time * 10.0).sin() * 1.5;
                    draw_sandworm_head(base_x + head_wave, base_y, head_dir);
                } else {
                    // 身体节：根据方向决定波动方向
                    let (offset_x, offset_y) = if i > 0 && i < snake.len() - 1 {
                        // 中间节：垂直于移动方向波动
                        let prev = snake[i - 1];
                        let dx = seg.x - prev.x;
                        if dx != 0 {
                            (0.0, wave_offset) // 水平移动时垂直波动
                        } else {
                            (wave_offset, 0.0) // 垂直移动时水平波动
                        }
                    } else {
                        (0.0, 0.0)
                    };
                    
                    // 绘制带斑驳纹理的身体
                    draw_sandworm_segment(
                        base_x + size_offset + offset_x,
                        base_y + size_offset + offset_y,
                        size,
                        segment_color,
                        seg.x,
                        seg.y,
                        game_time
                    );
                }
            }
        } else if buff_state.sandworm_phase == SandwormPhase::FilledFlashing {
            // 填满后闪烁变红
            let flash_speed = 10.0;
            let flash = (game_time * flash_speed).sin() * 0.5 + 0.5;
            let sand = sandworm_body_color();
            let red = sandworm_consume_color();
            // 从沙色渐变到红色
            let color = Color {
                r: sand.r + (red.r - sand.r) * flash,
                g: sand.g + (red.g - sand.g) * flash,
                b: sand.b + (red.b - sand.b) * flash,
                a: 1.0,
            };
            for seg in snake.iter() {
                let x = seg.x as f32 * CELL;
                let y = seg.y as f32 * CELL;
                draw_rectangle(x, y, CELL, CELL, color);
            }
        } else if buff_state.sandworm_phase == SandwormPhase::Consuming {
            // 吞噬阶段：坍缩动画
            let center = buff_state.sandworm_collapse_center;
            let progress = buff_state.sandworm_collapse_progress;
            // 使用缓动函数让坍缩更自然（先慢后快）
            let eased_progress = progress * progress * (3.0 - 2.0 * progress);
            
            for original_pos in buff_state.sandworm_collapse_positions.iter() {
                // 计算当前位置（从原始位置向中心移动）
                let current_x = original_pos.x + (center.x - original_pos.x) * eased_progress;
                let current_y = original_pos.y + (center.y - original_pos.y) * eased_progress;
                
                // 大小随进度缩小
                let size = CELL * (1.0 - eased_progress * 0.8);
                
                // 颜色：红色，越靠近中心越亮
                let dist_to_center = ((original_pos.x - center.x).powi(2) + (original_pos.y - center.y).powi(2)).sqrt();
                let max_dist = (GRID_W as f32 * CELL).max(GRID_H as f32 * CELL);
                let brightness = 1.0 - (dist_to_center / max_dist) * 0.5;
                let color = Color {
                    r: 0.8 * brightness + 0.2,
                    g: 0.1 * brightness,
                    b: 0.1 * brightness,
                    a: 1.0 - eased_progress * 0.3,
                };
                
                draw_rectangle(current_x - size / 2.0, current_y - size / 2.0, size, size, color);
            }
        } else {
            // 正常渲染（带受伤闪烁效果）
            let damage_flash = if damage_state.phase == DamagePhase::Flashing {
                // 快速闪烁：使用正弦波产生闪烁效果
                (game_time * 20.0).sin() > 0.0
            } else {
                false
            };
            
            for (i, seg) in snake.iter().enumerate() {
                let base_color = if i == 0 { GREEN } else { LIME };
                // 根据状态选择颜色
                let color = if damage_flash {
                    Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 } // 红色（受伤）
                } else if buff_state.frozen {
                    Color { r: 0.6, g: 0.9, b: 1.0, a: 1.0 } // 浅蓝色（冰冻）
                } else if buff_state.ghost_active {
                    Color { r: base_color.r, g: base_color.g, b: base_color.b, a: 0.5 } // 半透明（幽灵）
                } else if buff_state.speed_active {
                    Color { r: 0.3, g: 0.7, b: 1.0, a: 1.0 } // 蓝色（速度）
                } else if buff_state.slow_active {
                    Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 } // 橙色（减速）
                } else if buff_state.dizzy_active {
                    // 眩晕时颜色闪烁
                    let dizzy_flash = (game_time * 15.0).sin() * 0.3 + 0.7;
                    Color { r: 0.8 * dizzy_flash, g: 1.0 * dizzy_flash, b: 0.3, a: 1.0 } // 黄绿色闪烁
                } else if buff_state.slime_active {
                    Color { r: 0.3, g: 0.8, b: 0.3, a: 1.0 } // 绿色（粘液）
                } else {
                    base_color
                };
                
                let from = prev_snake.get(i).unwrap_or(seg);
                let gx = from.x as f32;
                let gy = from.y as f32;
                let mut dx = seg.x as f32 - gx;
                let mut dy = seg.y as f32 - gy;
                if wrap && !buff_state.sandworm_active {
                    let gw = GRID_W as f32;
                    let gh = GRID_H as f32;
                    if dx > gw * 0.5 { dx -= gw; }
                    if dx < -gw * 0.5 { dx += gw; }
                    if dy > gh * 0.5 { dy -= gh; }
                    if dy < -gh * 0.5 { dy += gh; }
                }
                
                // 计算基础位置
                let (mut x, mut y) = if buff_state.frozen {
                    // 冰冻时不使用插值，保持静止
                    (seg.x as f32 * CELL, seg.y as f32 * CELL)
                } else {
                    ((gx + dx * blend) * CELL, (gy + dy * blend) * CELL)
                };
                
                // 减速时的抽搐效果
                if buff_state.slow_active {
                    let twitch = (game_time * 30.0 + i as f32 * 2.0).sin() * 2.0;
                    x += twitch;
                    y += (game_time * 25.0 + i as f32 * 1.5).cos() * 1.5;
                }
                
                // 眩晕时的摇晃效果
                if buff_state.dizzy_active {
                    let wobble = (game_time * 12.0 + i as f32 * 0.8).sin() * 3.0;
                    x += wobble;
                    y += (game_time * 10.0 + i as f32 * 1.2).cos() * 2.0;
                }
                
                // 粘液时的拖拽抖动效果
                if buff_state.slime_active {
                    // 身体节越靠后，抖动越大（拖拽感）
                    let drag_factor = (i as f32 / snake.len() as f32) * 4.0;
                    let jitter_x = (game_time * 20.0 + i as f32 * 3.0).sin() * drag_factor;
                    let jitter_y = (game_time * 18.0 + i as f32 * 2.5).cos() * drag_factor;
                    x += jitter_x;
                    y += jitter_y;
                }
                
                draw_rectangle(x, y, CELL, CELL, color);
                
                // 冰冻时绘制冰晶覆盖效果
                if buff_state.frozen {
                    draw_ice_crystal(x, y, game_time, seg.x, seg.y);
                }
                
                // 粘液时绘制粘液滴落效果
                if buff_state.slime_active && i > 0 && (i + (game_time * 5.0) as usize) % 3 == 0 {
                    let drip_y = (game_time * 8.0 + i as f32).sin().abs() * 5.0;
                    let drip_color = Color { r: 0.2, g: 0.6, b: 0.2, a: 0.6 };
                    draw_rectangle(x + CELL * 0.3, y + CELL + drip_y, 4.0, 6.0, drip_color);
                }
            }
        }
        
        // 绘制粒子
        draw_particles(&particles);

        // 覆盖层与结束音效（只触发一次）
        if state == GameState::GameOver {
            if score > high_score { high_score = score; }
            if !over_once { play_sound(&over_snd, PlaySoundParams { looped: false, volume: 0.7 }); over_once = true; }
            draw_overlay("Game Over", "Enter/R to restart, Esc to quit");
        }
        if state == GameState::Paused {
            draw_overlay("Paused", "Space to resume, Enter/R to restart");
        }

        // HUD 文本：最后绘制以保证不被遮挡
        draw_text(&format!("Score: {}", score), 8.0, 18.0, 24.0, WHITE);
        draw_text(&format!("Best: {}", high_score), 140.0, 18.0, 24.0, YELLOW);
        
        // 显示Combo
        if combo_state.count > 1 && combo_state.display_timer > 0.0 {
            let combo_color = match combo_state.count {
                2 => YELLOW,
                3 => ORANGE,
                4 => Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 },
                _ => RED,
            };
            let combo_text = format!("COMBO x{}!", combo_state.count);
            let size = 28.0 + (combo_state.count as f32 * 2.0).min(10.0);
            draw_text(&combo_text, 280.0, 18.0, size, combo_color);
        }
        
        // 显示当前Buff状态
        let mut buff_y = 60.0;
        if buff_state.shield_active {
            draw_text(&format!("Shield: {:.1}s", buff_state.shield_timer), 8.0, buff_y, 18.0, GOLD);
            buff_y += 18.0;
        }
        if buff_state.speed_active {
            draw_text(&format!("Speed: {:.1}s", buff_state.speed_timer), 8.0, buff_y, 18.0, BLUE);
            buff_y += 18.0;
        }
        if buff_state.ghost_active {
            draw_text(&format!("Ghost: {:.1}s", buff_state.ghost_timer), 8.0, buff_y, 18.0, WHITE);
            buff_y += 18.0;
        }
        if buff_state.frozen {
            draw_text(&format!("FROZEN: {:.1}s", buff_state.freeze_timer), 8.0, buff_y, 18.0, SKYBLUE);
        }
        draw_text("[Arrows] Move  [Space] Pause  [Enter/R] Restart  [W] Wrap", 8.0, 40.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}

/// 生成不与蛇身重叠的随机食物位置
fn spawn_food(snake: &Vec<IVec2>, rng: &mut ThreadRng) -> IVec2 {
    loop {
        let p = ivec2(rng.gen_range(0..GRID_W), rng.gen_range(0..GRID_H));
        if !snake.iter().any(|&s| s == p) { return p; }
    }
}

/// 重开：重置蛇、方向、食物与分数
fn restart(snake: &mut Vec<IVec2>, dir: &mut IVec2, rng: &mut ThreadRng, food: &mut IVec2, score: &mut u32) {
    *snake = vec![ivec2(GRID_W / 2, GRID_H / 2), ivec2(GRID_W / 2 - 1, GRID_H / 2), ivec2(GRID_W / 2 - 2, GRID_H / 2)];
    *dir = ivec2(1, 0);
    *food = spawn_food(snake, rng);
    *score = 0;
}

// ============================================================================
// 特殊果实系统
// ============================================================================
// 
// 特殊果实系统为游戏增加了丰富的策略性和趣味性。
// 
// ## 设计理念
// 特殊果实分为两大类：陷阱果实（负面效果）和功能果实（正面效果）。
// 陷阱果实增加游戏难度和紧张感，功能果实作为奖励机制激励玩家。
// 
// ## 果实生成规则
// - 陷阱果实：每3秒有40%概率生成，存在5秒后自动消失
// - 功能果实：蛇长>=10时解锁，每帧1.5%概率生成，场上同时只能存在一个
// 
// ## 陷阱果实类型分布（总计100%）
// | 类型 | 概率 | 颜色 | 效果 | 视觉反馈 |
// |------|------|------|------|----------|
// | Trap | 35% | 紫色 | 蛇身-2格 | 红色闪烁+血迹+碎裂粒子 |
// | Freeze | 15% | 浅蓝 | 冰冻2秒 | 寒气粒子+冰晶覆盖 |
// | Slow | 20% | 橙色 | 减速5秒 | 蛇身抽搐+橙色染色 |
// | Dizzy | 15% | 黄绿 | 眩晕5秒 | 摇晃+颜色闪烁 |
// | Slime | 15% | 绿色 | 粘液5秒 | 拖拽抖动+粘液滴落 |
// 
// ## 功能果实类型分布（总计100%）
// | 类型 | 概率 | 颜色 | 存在时长 | 效果 | 视觉反馈 |
// |------|------|------|----------|------|----------|
// | Shield | 30% | 金色 | 8秒 | 10秒无敌 | 金色脉动光环 |
// | Speed | 25% | 蓝色 | 8秒 | 5秒加速 | 蓝色残影 |
// | Ghost | 20% | 白色 | 8秒 | 5秒穿墙 | 半透明+白色光环 |
// | Reverse | 15% | 粉色 | 5秒 | 头尾互换 | 无 |
// | Sandworm | 10% | 沙色 | 3秒 | 沙虫模式 | 多阶段变身动画 |
// 
// ## 免疫机制
// 护盾模式、沙虫模式、幽灵模式激活时，免疫所有陷阱效果。
// ============================================================================

// === 果实生命周期常量 ===

/// 陷阱果实存在时长（秒）
/// 
/// 所有陷阱果实（Trap/Freeze/Slow/Dizzy/Slime）共用此时长。
/// 5秒的设计是为了给玩家足够的反应时间，同时保持紧张感。
const TRAP_LIFETIME: f32 = 5.0;

/// 护盾果实存在时长（秒）
/// 
/// 大多数功能果实使用此时长（Shield/Speed/Ghost）。
/// 8秒足够让玩家有机会获取，但不会让场上果实过多。
const SHIELD_FRUIT_LIFETIME: f32 = 8.0;

/// 沙虫果实存在时长（秒）
/// 
/// 沙虫果实时长较短（3秒），因为：
/// 1. 沙虫模式效果极其强大（直接胜利）
/// 2. 增加稀有感和紧迫感
/// 3. 需要玩家快速决策是否冒险获取
const SANDWORM_FRUIT_LIFETIME: f32 = 3.0;

// === Buff持续时间常量 ===

/// 护盾Buff持续时间（秒）
/// 
/// 10秒的无敌时间足够玩家安全穿越危险区域，
/// 但不会让游戏变得过于简单。
const SHIELD_DURATION: f32 = 10.0;

/// 沙虫模式移动间隔（秒）
/// 
/// 0.02秒（50FPS）的移动速度产生流畅的填充效果。
/// 填满32x24=768格需要约15秒。
const SANDWORM_TICK_INTERVAL: f32 = 0.02;

/// 生成不与蛇身和现有果实重叠的随机位置
/// 
/// ## 算法
/// 最多尝试100次随机位置，找到空闲位置则返回。
/// 如果100次都找不到（场地几乎被占满），返回None。
/// 
/// ## 参数
/// - `snake`: 当前蛇身位置列表
/// - `fruits`: 当前场上的果实列表
/// - `rng`: 随机数生成器
/// 
/// ## 返回
/// - `Some(pos)`: 找到的空闲位置
/// - `None`: 无法找到空闲位置
fn spawn_position(snake: &[IVec2], fruits: &[Fruit], rng: &mut ThreadRng) -> Option<IVec2> {
    for _ in 0..100 {
        let p = ivec2(rng.gen_range(0..GRID_W), rng.gen_range(0..GRID_H));
        if !snake.iter().any(|&s| s == p) && !fruits.iter().any(|f| f.pos == p) {
            return Some(p);
        }
    }
    None
}

/// 生成陷阱果实
/// 
/// 随机生成各种陷阱类型：
/// - Trap(35%): 蛇身减少2格，触发受伤动画
/// - Freeze(15%): 冻住2秒无法移动
/// - Slow(20%): 减速5秒，蛇身抽搐
/// - Dizzy(15%): 眩晕5秒，方向随机偏移
/// - Slime(15%): 粘液5秒，移动拖拽抖动
fn spawn_trap_fruit(snake: &[IVec2], fruits: &[Fruit], game_time: f32, rng: &mut ThreadRng) -> Option<Fruit> {
    spawn_position(snake, fruits, rng).map(|pos| {
        // 随机选择陷阱类型
        let roll = rng.gen_range(0..100);
        let fruit_type = if roll < 35 {
            FruitType::Trap      // 35%
        } else if roll < 50 {
            FruitType::Freeze    // 15%
        } else if roll < 70 {
            FruitType::Slow      // 20%
        } else if roll < 85 {
            FruitType::Dizzy     // 15%
        } else {
            FruitType::Slime     // 15%
        };
        Fruit {
            pos,
            fruit_type,
            spawn_time: game_time,
            lifetime: TRAP_LIFETIME,
        }
    })
}

/// 幸运方块存在时长（秒）
const LUCKY_LIFETIME: f32 = 6.0;

/// 生成幸运方块果实
/// 
/// 幸运方块是特殊果实，吃到后随机触发增益或负面效果。
/// 
/// ## 效果概率
/// - 50% 增益效果（护盾/速度/幽灵/额外分数/蛇身增长）
/// - 50% 负面效果（冰冻/减速/眩晕/粘液/蛇身减少）
fn spawn_lucky_fruit(snake: &[IVec2], fruits: &[Fruit], game_time: f32, rng: &mut ThreadRng) -> Option<Fruit> {
    spawn_position(snake, fruits, rng).map(|pos| {
        Fruit {
            pos,
            fruit_type: FruitType::Lucky,
            spawn_time: game_time,
            lifetime: LUCKY_LIFETIME,
        }
    })
}

/// 更新果实列表，移除过期果实
/// 
/// 检查所有有时限的果实，移除已过期的。
/// 
/// ## 返回
/// 过期果实的位置列表（可用于生成消失特效）
fn update_fruits(fruits: &mut Vec<Fruit>, game_time: f32) -> Vec<IVec2> {
    let mut expired = vec![];
    fruits.retain(|f| {
        if f.lifetime > 0.0 && game_time - f.spawn_time >= f.lifetime {
            expired.push(f.pos);
            false
        } else {
            true
        }
    });
    expired
}

/// 计算进度条值
/// 
/// 用于显示果实剩余时间的进度条。
/// 
/// ## 返回
/// 0.0（已过期）到 1.0（刚生成）之间的值
fn calc_progress(spawn_time: f32, lifetime: f32, current_time: f32) -> f32 {
    if lifetime <= 0.0 { return 1.0; }
    (1.0 - (current_time - spawn_time) / lifetime).max(0.0)
}

/// 处理陷阱果实消费：返回 true 表示游戏结束（已被受伤动画系统替代）
#[allow(dead_code)]
fn consume_trap_fruit(snake: &mut Vec<IVec2>, buff: &BuffState) -> bool {
    // 如果有护盾或沙虫模式，免疫陷阱效果
    if buff.shield_active || buff.sandworm_active {
        return false;
    }
    // 蛇长度 <= 3 时吃陷阱会死亡
    if snake.len() <= 3 {
        return true;
    }
    // 减少2格
    snake.pop();
    snake.pop();
    false
}

/// 绘制进度条
fn draw_progress_bar(pos: IVec2, progress: f32) {
    let x = pos.x as f32 * CELL;
    let y = pos.y as f32 * CELL - 4.0;
    let w = CELL;
    let h = 3.0;
    // 背景
    draw_rectangle(x, y, w, h, DARKGRAY);
    // 进度
    let prog_color = if progress > 0.3 { YELLOW } else { RED };
    draw_rectangle(x, y, w * progress, h, prog_color);
}

// ============================================================================
// 粒子特效系统
// ============================================================================

/// 粒子最大存活时间
#[allow(dead_code)]
const PARTICLE_LIFETIME: f32 = 0.5;

/// 生成爆发粒子
#[allow(dead_code)]
fn spawn_burst_particles(particles: &mut Vec<Particle>, pos: Vec2, count: usize, color: Color, rng: &mut ThreadRng) {
    for _ in 0..count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime: PARTICLE_LIFETIME,
            max_lifetime: PARTICLE_LIFETIME,
            size: rng.gen_range(3.0..6.0),
        });
    }
}

/// 更新粒子：移动并减少生命周期
fn update_particles(particles: &mut Vec<Particle>, dt: f32) {
    for p in particles.iter_mut() {
        p.pos += p.vel * dt;
        p.vel *= 0.95; // 阻尼
        p.lifetime -= dt;
    }
    particles.retain(|p| p.lifetime > 0.0);
}

/// 绘制所有粒子
fn draw_particles(particles: &[Particle]) {
    for p in particles {
        let alpha = (p.lifetime / p.max_lifetime).clamp(0.0, 1.0);
        let size = p.size * alpha;
        let color = Color { a: alpha, ..p.color };
        draw_rectangle(p.pos.x - size / 2.0, p.pos.y - size / 2.0, size, size, color);
    }
}

/// 在陷阱果实消费时生成破碎粒子（在尾部位置）（已被受伤动画系统替代）
#[allow(dead_code)]
fn spawn_trap_consumption_particles(particles: &mut Vec<Particle>, tail_positions: &[IVec2], rng: &mut ThreadRng) {
    let trap_color = FruitType::Trap.color();
    for pos in tail_positions {
        let center = vec2(pos.x as f32 * CELL + CELL / 2.0, pos.y as f32 * CELL + CELL / 2.0);
        spawn_burst_particles(particles, center, 8, trap_color, rng);
    }
}

/// 生成鲜血喷溅爆炸粒子（沙虫坍缩完成时）
fn spawn_blood_explosion(particles: &mut Vec<Particle>, center: Vec2, rng: &mut ThreadRng) {
    // 生成大量红色粒子，模拟鲜血喷溅
    let blood_colors = [
        Color { r: 0.8, g: 0.1, b: 0.1, a: 1.0 }, // 深红
        Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 }, // 亮红
        Color { r: 0.6, g: 0.0, b: 0.0, a: 1.0 }, // 暗红
        Color { r: 1.0, g: 0.4, b: 0.3, a: 1.0 }, // 橙红
    ];
    
    // 生成多层爆炸粒子
    for _ in 0..150 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(100.0..400.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let color = blood_colors[rng.gen_range(0..blood_colors.len())];
        let lifetime = rng.gen_range(0.8..1.5);
        
        particles.push(Particle {
            pos: center,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(4.0..12.0),
        });
    }
}

// ============================================================================
// 功能果实系统
// ============================================================================
// 
// 功能果实是游戏的核心奖励机制，提供各种增益效果。
// 蛇长度达到10时解锁，场上同时只能存在一个功能果实。
// ============================================================================

/// 检查是否应该解锁功能果实
/// 
/// 当蛇长度>=10时返回true，解锁功能果实的生成。
/// 这个阈值设计是为了让玩家在游戏初期专注于基础玩法。
fn should_enable_power_fruit(snake_len: usize) -> bool {
    snake_len >= 10
}

/// 生成功能果实
/// 
/// 随机选择一种功能果实类型并生成。
/// 
/// ## 类型概率分布
/// | 类型 | 概率 | 存在时长 | 效果 |
/// |------|------|----------|------|
/// | Shield | 30% | 8秒 | 10秒无敌 |
/// | Speed | 25% | 8秒 | 5秒加速 |
/// | Ghost | 20% | 8秒 | 5秒穿墙 |
/// | Reverse | 15% | 5秒 | 头尾互换 |
/// | Sandworm | 10% | 3秒 | 触发沙虫模式 |
/// 
/// ## 生成限制
/// - 场上已有功能果实时不生成
/// - 无法找到空闲位置时不生成
fn spawn_power_fruit(snake: &[IVec2], fruits: &[Fruit], game_time: f32, rng: &mut ThreadRng) -> Option<Fruit> {
    // 检查是否已有功能果实
    if fruits.iter().any(|f| f.fruit_type.is_power()) {
        return None;
    }
    
    spawn_position(snake, fruits, rng).map(|pos| {
        // 随机选择功能果实类型（加权随机）
        let roll = rng.gen_range(0..100);
        let fruit_type = if roll < 30 {
            FruitType::Shield    // 30%
        } else if roll < 55 {
            FruitType::Speed     // 25%
        } else if roll < 75 {
            FruitType::Ghost     // 20%
        } else if roll < 90 {
            FruitType::Reverse   // 15%
        } else {
            FruitType::Sandworm  // 10%
        };
        
        // 不同类型有不同的存在时长
        let lifetime = match fruit_type {
            FruitType::Sandworm => SANDWORM_FRUIT_LIFETIME, // 3秒（稀有，时间短）
            FruitType::Reverse => 5.0,                       // 5秒
            _ => SHIELD_FRUIT_LIFETIME,                      // 8秒
        };
        
        Fruit {
            pos,
            fruit_type,
            spawn_time: game_time,
            lifetime,
        }
    })
}

/// 激活护盾模式
/// 
/// 设置护盾状态为激活，并初始化10秒计时器。
/// 护盾激活期间免疫所有陷阱效果。
fn activate_shield(buff: &mut BuffState) {
    buff.shield_active = true;
    buff.shield_timer = SHIELD_DURATION;
}

/// 更新护盾计时器
/// 
/// 每帧调用，减少护盾剩余时间。
/// 时间耗尽时自动关闭护盾。
fn update_shield(buff: &mut BuffState, dt: f32) {
    if buff.shield_active {
        buff.shield_timer -= dt;
        if buff.shield_timer <= 0.0 {
            buff.shield_active = false;
            buff.shield_timer = 0.0;
        }
    }
}

/// 绘制护盾效果
/// 
/// 在蛇身周围绘制金色脉动光环。
/// 使用正弦波产生呼吸效果。
fn draw_shield_effect(snake: &[IVec2], time: f32) {
    let pulse = (time * 4.0).sin() * 0.2 + 0.8; // 脉动系数
    let gold = Color { r: 1.0, g: 0.84, b: 0.0, a: 0.3 * pulse };
    
    for seg in snake {
        let x = seg.x as f32 * CELL - 2.0;
        let y = seg.y as f32 * CELL - 2.0;
        let size = CELL + 4.0;
        draw_rectangle(x, y, size, size, gold);
    }
}

// ============================================================================
// 沙虫果实系统
// ============================================================================
// 
// 沙虫模式是游戏中最复杂的特殊效果，包含多阶段动画：
// 
// ## 阶段流程
// 1. Flashing (1.5秒): 蛇原地闪烁，提示变身开始
// 2. Transforming: 从头到尾逐节变成沙色
// 3. Exiting: 沙虫向最近边界移动直到完全消失
// 4. Filling: 从(0,0)开始顺时针螺旋填满整个游戏盘
// 5. FilledFlashing (1.5秒): 填满后闪烁变红
// 6. Consuming: 所有格子向中心坍缩，产生爆炸效果
// 
// ## 胜利条件
// 完成所有阶段后游戏胜利，显示"WIN"画面
// ============================================================================

/// 生成顺时针螺旋路径
/// 
/// 从(0,0)开始，沿边界顺时针移动，然后螺旋向内。
/// 用于沙虫模式的填充阶段。
/// 
/// ## 路径示例（4x3网格）
/// ```text
/// 1  2  3  4
/// 10 11 12 5
/// 9  8  7  6
/// ```
/// 
/// ## 返回
/// 包含所有网格位置的向量，按访问顺序排列
fn generate_clockwise_path() -> Vec<IVec2> {
    let mut path = Vec::with_capacity((GRID_W * GRID_H) as usize);
    let mut left = 0;
    let mut right = GRID_W - 1;
    let mut top = 0;
    let mut bottom = GRID_H - 1;
    
    while left <= right && top <= bottom {
        // 上边：从左到右
        for x in left..=right {
            path.push(ivec2(x, top));
        }
        top += 1;
        
        // 右边：从上到下
        for y in top..=bottom {
            path.push(ivec2(right, y));
        }
        right -= 1;
        
        // 下边：从右到左
        if top <= bottom {
            for x in (left..=right).rev() {
                path.push(ivec2(x, bottom));
            }
            bottom -= 1;
        }
        
        // 左边：从下到上
        if left <= right {
            for y in (top..=bottom).rev() {
                path.push(ivec2(left, y));
            }
            left += 1;
        }
    }
    
    path
}

// === 沙虫变身动画常量 ===
/// 闪烁阶段持续时间（秒）
const SANDWORM_FLASH_DURATION: f32 = 1.5;
/// 每节变色的间隔时间（秒）
const SANDWORM_TRANSFORM_INTERVAL: f32 = 0.08;
/// 填满后闪烁变红的持续时间（秒）
const SANDWORM_FILLED_FLASH_DURATION: f32 = 1.5;

/// 获取沙虫身体颜色（沙色）
fn sandworm_body_color() -> Color {
    Color { r: 0.76, g: 0.60, b: 0.42, a: 1.0 }
}

/// 获取沙虫头部颜色（深沙色，比身体深）
fn sandworm_head_color() -> Color {
    Color { r: 0.55, g: 0.40, b: 0.25, a: 1.0 }
}

/// 绘制带斑驳纹理的沙虫身体格子
fn draw_sandworm_segment(x: f32, y: f32, size: f32, base_color: Color, seg_x: i32, seg_y: i32, time: f32) {
    // 先绘制底色
    draw_rectangle(x, y, size, size, base_color);
    
    // 在格子内绘制多个小斑点
    let spot_count = 4; // 每个格子内的斑点数
    let spot_size = size * 0.25; // 斑点大小
    
    for i in 0..spot_count {
        // 使用位置生成伪随机斑点位置
        let hash1 = ((seg_x * 7919 + seg_y * 104729 + i * 31) % 100) as f32 / 100.0;
        let hash2 = ((seg_x * 104729 + seg_y * 7919 + i * 47) % 100) as f32 / 100.0;
        let hash3 = ((seg_x * 31 + seg_y * 17 + i * 7919) % 100) as f32 / 100.0;
        
        // 斑点位置（在格子内随机分布）
        let spot_x = x + hash1 * (size - spot_size);
        let spot_y = y + hash2 * (size - spot_size);
        
        // 斑点颜色（深色）
        let dark_factor = 0.3 + hash3 * 0.3;
        let spot_color = Color {
            r: base_color.r * dark_factor,
            g: base_color.g * dark_factor,
            b: base_color.b * dark_factor,
            a: 0.7 + (time * 3.0 + hash1 * 10.0).sin() * 0.2, // 轻微闪烁
        };
        
        // 只绘制部分斑点（约60%的概率）
        if hash3 < 0.6 {
            draw_rectangle(spot_x, spot_y, spot_size * (0.5 + hash3), spot_size * (0.5 + hash1), spot_color);
        }
    }
}

/// 斑点红色（吞噬阶段的颜色）
fn sandworm_consume_color() -> Color {
    Color { r: 0.8, g: 0.2, b: 0.2, a: 1.0 } // 斑点红色
}

/// 绘制沙虫果实（带斑驳纹理）
fn draw_sandworm_fruit(x: f32, y: f32, time: f32, pos_x: i32, pos_y: i32) {
    let base_color = sandworm_body_color();
    
    // 先绘制底色
    draw_rectangle(x, y, CELL, CELL, base_color);
    
    // 在格子内绘制多个小斑点（与身体类似但稍有不同）
    let spot_count = 5; // 果实斑点稍多一些
    let spot_size = CELL * 0.22;
    
    for i in 0..spot_count {
        // 使用位置生成伪随机斑点位置（加入不同的种子使果实斑点与身体不同）
        let hash1 = ((pos_x * 12347 + pos_y * 98765 + i * 53) % 100) as f32 / 100.0;
        let hash2 = ((pos_x * 98765 + pos_y * 12347 + i * 71) % 100) as f32 / 100.0;
        let hash3 = ((pos_x * 53 + pos_y * 29 + i * 12347) % 100) as f32 / 100.0;
        
        // 斑点位置（在格子内随机分布）
        let spot_x = x + hash1 * (CELL - spot_size);
        let spot_y = y + hash2 * (CELL - spot_size);
        
        // 斑点颜色（深色）
        let dark_factor = 0.25 + hash3 * 0.35;
        let spot_color = Color {
            r: base_color.r * dark_factor,
            g: base_color.g * dark_factor,
            b: base_color.b * dark_factor,
            a: 0.75 + (time * 4.0 + hash1 * 8.0).sin() * 0.15, // 轻微闪烁
        };
        
        // 绘制大部分斑点（约70%的概率）
        if hash3 < 0.7 {
            draw_rectangle(spot_x, spot_y, spot_size * (0.6 + hash3 * 0.4), spot_size * (0.6 + hash1 * 0.4), spot_color);
        }
    }
    
    // 添加轻微的脉动边框效果，让果实更显眼
    let pulse = (time * 3.0).sin() * 0.3 + 0.7;
    let border_color = Color {
        r: base_color.r * 0.6,
        g: base_color.g * 0.6,
        b: base_color.b * 0.6,
        a: 0.5 * pulse,
    };
    let border_width = 2.0;
    // 上边
    draw_rectangle(x, y, CELL, border_width, border_color);
    // 下边
    draw_rectangle(x, y + CELL - border_width, CELL, border_width, border_color);
    // 左边
    draw_rectangle(x, y, border_width, CELL, border_color);
    // 右边
    draw_rectangle(x + CELL - border_width, y, border_width, CELL, border_color);
}

/// 绘制沙虫头部（带嘴巴和牙齿）
fn draw_sandworm_head(x: f32, y: f32, dir: IVec2) {
    let head_color = sandworm_head_color();
    let tooth_color = Color { r: 0.95, g: 0.95, b: 0.85, a: 1.0 }; // 牙齿颜色（米白色）
    let mouth_color = Color { r: 0.3, g: 0.15, b: 0.1, a: 1.0 }; // 嘴巴内部（深棕色）
    
    // 头部比身体宽一点（向两侧扩展2像素）
    let expand = 2.0;
    draw_rectangle(x - expand, y - expand, CELL + expand * 2.0, CELL + expand * 2.0, head_color);
    
    // 根据方向绘制嘴巴和牙齿
    let tooth_size = 4.0;
    let tooth_gap = 6.0;
    
    match (dir.x, dir.y) {
        (1, 0) => { // 向右
            // 嘴巴（右侧）
            let mouth_x = x + CELL - 2.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            // 上牙齿
            draw_rectangle(mouth_x + 2.0, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + 2.0 + tooth_gap, tooth_size, tooth_size, tooth_color);
            // 下牙齿
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0 - tooth_gap, tooth_size, tooth_size, tooth_color);
        }
        (-1, 0) => { // 向左
            let mouth_x = x - 4.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            draw_rectangle(mouth_x, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + 2.0 + tooth_gap, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x, y + CELL - 6.0 - tooth_gap, tooth_size, tooth_size, tooth_color);
        }
        (0, 1) => { // 向下
            let mouth_y = y + CELL - 2.0;
            draw_rectangle(x + 4.0, mouth_y, CELL - 8.0, 6.0, mouth_color);
            draw_rectangle(x + 2.0, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + 2.0 + tooth_gap, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0 - tooth_gap, mouth_y + 2.0, tooth_size, tooth_size, tooth_color);
        }
        (0, -1) => { // 向上
            let mouth_y = y - 4.0;
            draw_rectangle(x + 4.0, mouth_y, CELL - 8.0, 6.0, mouth_color);
            draw_rectangle(x + 2.0, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + 2.0 + tooth_gap, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0, mouth_y, tooth_size, tooth_size, tooth_color);
            draw_rectangle(x + CELL - 6.0 - tooth_gap, mouth_y, tooth_size, tooth_size, tooth_color);
        }
        _ => {
            // 默认向右
            let mouth_x = x + CELL - 2.0;
            draw_rectangle(mouth_x, y + 4.0, 6.0, CELL - 8.0, mouth_color);
            draw_rectangle(mouth_x + 2.0, y + 2.0, tooth_size, tooth_size, tooth_color);
            draw_rectangle(mouth_x + 2.0, y + CELL - 6.0, tooth_size, tooth_size, tooth_color);
        }
    }
}

/// 激活沙虫模式（开始变身动画）
fn activate_sandworm(buff: &mut BuffState, snake: &[IVec2], dir: IVec2) {
    buff.sandworm_active = true;
    buff.sandworm_phase = SandwormPhase::Flashing;
    buff.sandworm_phase_timer = 0.0;
    buff.sandworm_transform_index = 0;
    buff.sandworm_original_snake = snake.to_vec();
    buff.sandworm_original_dir = dir;
    // 路径和索引在变身完成后设置
    buff.sandworm_path = vec![];
    buff.sandworm_index = 0;
    buff.sandworm_tick = 0.0;
}

/// 更新沙虫变身动画：返回 (是否在变身/特殊阶段, 是否游戏胜利)
fn update_sandworm_transform(buff: &mut BuffState, snake: &mut Vec<IVec2>, dt: f32) -> (bool, bool) {
    if !buff.sandworm_active {
        return (false, false);
    }
    
    
    match buff.sandworm_phase {
        SandwormPhase::Flashing => {
            buff.sandworm_phase_timer += dt;
            if buff.sandworm_phase_timer >= SANDWORM_FLASH_DURATION {
                // 进入变色阶段
                buff.sandworm_phase = SandwormPhase::Transforming;
                buff.sandworm_phase_timer = 0.0;
                buff.sandworm_transform_index = 0;
            }
            (true, false)
        }
        SandwormPhase::Transforming => {
            buff.sandworm_phase_timer += dt;
            // 每隔一段时间变色一节
            while buff.sandworm_phase_timer >= SANDWORM_TRANSFORM_INTERVAL {
                buff.sandworm_phase_timer -= SANDWORM_TRANSFORM_INTERVAL;
                buff.sandworm_transform_index += 1;
                
                // 所有节都变色完成
                if buff.sandworm_transform_index >= buff.sandworm_original_snake.len() {
                    // 进入退出阶段，计算最近边界方向
                    buff.sandworm_phase = SandwormPhase::Exiting;
                    buff.sandworm_tick = 0.0;
                    
                    // 计算头部到四个边界的距离，选择最近的
                    if let Some(head) = snake.first() {
                        let dist_left = head.x;
                        let dist_right = GRID_W - 1 - head.x;
                        let dist_top = head.y;
                        let dist_bottom = GRID_H - 1 - head.y;
                        
                        let min_dist = dist_left.min(dist_right).min(dist_top).min(dist_bottom);
                        
                        buff.sandworm_exit_dir = if min_dist == dist_left {
                            ivec2(-1, 0) // 向左
                        } else if min_dist == dist_right {
                            ivec2(1, 0) // 向右
                        } else if min_dist == dist_top {
                            ivec2(0, -1) // 向上
                        } else {
                            ivec2(0, 1) // 向下
                        };
                    }
                    return (true, false);
                }
            }
            (true, false)
        }
        SandwormPhase::Exiting => {
            // 退出阶段：蛇向边界移动直到完全消失
            buff.sandworm_tick += dt;
            
            while buff.sandworm_tick >= SANDWORM_TICK_INTERVAL * 2.0 {
                buff.sandworm_tick -= SANDWORM_TICK_INTERVAL * 2.0;
                
                // 移动蛇：添加新头部，移除尾部
                if let Some(head) = snake.first().cloned() {
                    let new_head = head + buff.sandworm_exit_dir;
                    snake.insert(0, new_head);
                }
                snake.pop();
                
                // 检查蛇是否完全离开屏幕
                let all_outside = snake.iter().all(|seg| {
                    seg.x < 0 || seg.x >= GRID_W || seg.y < 0 || seg.y >= GRID_H
                });
                
                if all_outside || snake.is_empty() {
                    // 蛇完全消失，进入填充阶段
                    buff.sandworm_phase = SandwormPhase::Filling;
                    buff.sandworm_path = generate_clockwise_path();
                    buff.sandworm_index = 1; // 从索引1开始，因为蛇已经在(0,0)
                    buff.sandworm_tick = 0.0;
                    // 重置蛇身为单个头部在 (0,0)
                    snake.clear();
                    snake.push(ivec2(0, 0));
                    return (true, false);
                }
            }
            (true, false)
        }
        SandwormPhase::Filling => {
            // 填充阶段：蛇不断增长，不移除尾部
            buff.sandworm_tick += dt;
            
            while buff.sandworm_tick >= SANDWORM_TICK_INTERVAL {
                buff.sandworm_tick -= SANDWORM_TICK_INTERVAL;
                
                // 检查是否填满
                if buff.sandworm_index >= buff.sandworm_path.len() {
                    // 填满了！进入填满闪烁阶段
                    buff.sandworm_phase = SandwormPhase::FilledFlashing;
                    buff.sandworm_phase_timer = 0.0;
                    buff.sandworm_transform_index = 0;
                    return (true, false);
                }
                
                let next_pos = buff.sandworm_path[buff.sandworm_index];
                snake.insert(0, next_pos); // 只增加头部，不移除尾部
                buff.sandworm_index += 1;
            }
            (true, false)
        }
        SandwormPhase::FilledFlashing => {
            // 填满后闪烁变红
            buff.sandworm_phase_timer += dt;
            if buff.sandworm_phase_timer >= SANDWORM_FILLED_FLASH_DURATION {
                // 进入吞噬阶段，初始化坍缩动画
                buff.sandworm_phase = SandwormPhase::Consuming;
                buff.sandworm_collapse_progress = 0.0;
                // 头部位置作为坍缩中心
                if let Some(head) = snake.first() {
                    buff.sandworm_collapse_center = vec2(
                        head.x as f32 * CELL + CELL / 2.0,
                        head.y as f32 * CELL + CELL / 2.0
                    );
                }
                // 记录所有格子的初始位置
                buff.sandworm_collapse_positions = snake.iter()
                    .map(|seg| vec2(seg.x as f32 * CELL + CELL / 2.0, seg.y as f32 * CELL + CELL / 2.0))
                    .collect();
            }
            (true, false)
        }
        SandwormPhase::Consuming => {
            // 吞噬阶段：坍缩动画
            let collapse_speed = 0.8; // 坍缩速度
            buff.sandworm_collapse_progress += dt * collapse_speed;
            
            if buff.sandworm_collapse_progress >= 1.0 {
                // 坍缩完成，游戏胜利
                buff.sandworm_active = false;
                buff.sandworm_phase = SandwormPhase::None;
                snake.clear();
                return (false, true); // 游戏胜利，需要生成爆炸粒子
            }
            (true, false)
        }
        _ => (false, false)
    }
}

/// 绘制沙虫变身动画
fn draw_sandworm_transform(snake: &[IVec2], buff: &BuffState, time: f32) {
    match buff.sandworm_phase {
        SandwormPhase::Flashing => {
            // 闪烁效果：整体闪烁，颜色在绿色和白色之间切换
            let flash_speed = 12.0; // 闪烁频率
            let flash = ((time * flash_speed).sin() * 0.5 + 0.5).powi(2);
            
            for (i, seg) in snake.iter().enumerate() {
                let base_color = if i == 0 { GREEN } else { LIME };
                // 混合白色实现闪烁
                let color = Color {
                    r: base_color.r + (1.0 - base_color.r) * flash,
                    g: base_color.g + (1.0 - base_color.g) * flash,
                    b: base_color.b + (1.0 - base_color.b) * flash,
                    a: 1.0,
                };
                let x = seg.x as f32 * CELL;
                let y = seg.y as f32 * CELL;
                draw_rectangle(x, y, CELL, CELL, color);
                
                // 添加光晕效果
                if flash > 0.5 {
                    let glow = Color { r: 1.0, g: 1.0, b: 0.8, a: (flash - 0.5) * 0.6 };
                    draw_rectangle(x - 2.0, y - 2.0, CELL + 4.0, CELL + 4.0, glow);
                }
            }
        }
        SandwormPhase::Transforming => {
            // 变色效果：从头到尾逐渐变成沙色
            let sand_color = sandworm_body_color();
            
            for (i, seg) in snake.iter().enumerate() {
                let x = seg.x as f32 * CELL;
                let y = seg.y as f32 * CELL;
                
                if i < buff.sandworm_transform_index {
                    // 已变色的部分：使用斑驳纹理
                    if i == 0 {
                        // 头部使用沙虫头部样式
                        let head_dir = if snake.len() > 1 {
                            let next = snake[1];
                            ivec2(seg.x - next.x, seg.y - next.y)
                        } else {
                            buff.sandworm_original_dir
                        };
                        draw_sandworm_head(x, y, head_dir);
                    } else {
                        // 身体使用斑驳纹理
                        draw_sandworm_segment(x, y, CELL, sand_color, seg.x, seg.y, time);
                    }
                } else if i == buff.sandworm_transform_index {
                    // 正在变色的部分（过渡效果）
                    let progress = buff.sandworm_phase_timer / SANDWORM_TRANSFORM_INTERVAL;
                    let base = if i == 0 { GREEN } else { LIME };
                    let target = if i == 0 { sandworm_head_color() } else { sand_color };
                    let color = Color {
                        r: base.r + (target.r - base.r) * progress,
                        g: base.g + (target.g - base.g) * progress,
                        b: base.b + (target.b - base.b) * progress,
                        a: 1.0,
                    };
                    draw_rectangle(x, y, CELL, CELL, color);
                    // 正在变色的节添加粒子效果
                    let glow = Color { r: 1.0, g: 0.9, b: 0.6, a: 0.4 };
                    draw_rectangle(x - 3.0, y - 3.0, CELL + 6.0, CELL + 6.0, glow);
                } else {
                    // 未变色的部分
                    let color = if i == 0 { GREEN } else { LIME };
                    draw_rectangle(x, y, CELL, CELL, color);
                }
            }
        }
        _ => {}
    }
}



/// 绘制游戏区域边框与半透明网格线（不影响碰撞与逻辑，仅视觉辅助）
fn draw_border_and_grid() {
    let w = GRID_W as f32 * CELL;
    let h = GRID_H as f32 * CELL;
    draw_rectangle_lines(0.0, 0.0, w, h, 2.0, WHITE);
    let gc = Color { r: 0.6, g: 0.6, b: 0.6, a: 0.2 };
    for x in 1..GRID_W { let xf = x as f32 * CELL; draw_line(xf, 0.0, xf, h, 1.0, gc); }
    for y in 1..GRID_H { let yf = y as f32 * CELL; draw_line(0.0, yf, w, yf, 1.0, gc); }
}

/// 绘制半透明覆盖层与居中文本标题/副标题
fn draw_overlay(title: &str, subtitle: &str) {
    let w = GRID_W as f32 * CELL;
    let h = GRID_H as f32 * CELL;
    draw_rectangle(0.0, 0.0, w, h, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 });
    let t = measure_text(title, None, 48, 1.0);
    let s = measure_text(subtitle, None, 24, 1.0);
    draw_text(title, (w - t.width) * 0.5, h * 0.5 - 10.0, 48.0, WHITE);
    draw_text(subtitle, (w - s.width) * 0.5, h * 0.5 + 30.0, 24.0, LIGHTGRAY);
}

/// 生成简单的单声道 16-bit PCM WAV 音频数据（正弦波）
fn make_tone_wav(freq: f32, dur: f32, vol: f32) -> Vec<u8> {
    let sr = 44100;
    let samples = (dur * sr as f32) as usize;
    let mut pcm: Vec<i16> = Vec::with_capacity(samples);
    for n in 0..samples {
        let t = n as f32 / sr as f32;
        let s = (2.0 * std::f32::consts::PI * freq * t).sin();
        let v = (s * (vol.clamp(0.0, 1.0)) * std::i16::MAX as f32) as i16;
        pcm.push(v);
    }
    let data_size = (pcm.len() * 2) as u32;
    let mut out: Vec<u8> = Vec::with_capacity(44 + data_size as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_size).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&(sr as u32).to_le_bytes());
    let byte_rate = sr as u32 * 2;
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    for s in pcm { out.extend_from_slice(&s.to_le_bytes()); }
    out
}

// ============================================================================
// 受伤动画系统
// ============================================================================
// 
// 当蛇吃到Trap果实时触发的受伤动画，模仿Minecraft的受伤效果。
// 
// ## 动画流程
// 1. **Flashing阶段** (0.6秒)
//    - 蛇身快速闪烁红色，共闪烁4次
//    - 每次闪烁周期0.15秒
//    - 视觉上提示玩家受到伤害
// 
// 2. **Crumbling阶段** (0.3秒)
//    - 尾巴一节一节碎裂消失
//    - 每0.15秒碎裂一节，共碎裂2节
//    - 每节碎裂时生成红色血液粒子
//    - 在碎裂位置留下血迹
// 
// ## 血迹系统
// - 血迹在地面停留5秒后淡出消失
// - 包含主血迹和随机溅射的小血滴
// - 增加游戏的视觉冲击力
// ============================================================================

// === 受伤动画常量 ===

/// 闪烁阶段总时长（秒）
const DAMAGE_FLASH_DURATION: f32 = 0.6;

/// 闪烁次数
/// 
/// 4次闪烁在0.6秒内完成，每次0.15秒
const DAMAGE_FLASH_COUNT: i32 = 4;

/// 每节尾巴碎裂的间隔时间（秒）
/// 
/// 0.15秒的间隔让碎裂效果清晰可见
const DAMAGE_CRUMBLE_INTERVAL: f32 = 0.15;

/// 血迹存在时间（秒）
/// 
/// 5秒后血迹完全淡出消失
const BLOOD_STAIN_LIFETIME: f32 = 5.0;

/// 开始受伤动画
/// 
/// 初始化受伤状态，记录需要移除的尾巴节。
/// 
/// ## 参数
/// - `damage`: 受伤状态结构体
/// - `snake`: 当前蛇身位置列表
/// 
/// ## 注意
/// 只有蛇长度>3时才会触发动画，否则直接游戏结束
fn start_damage_animation(damage: &mut DamageState, snake: &[IVec2]) {
    damage.active = true;
    damage.phase = DamagePhase::Flashing;
    damage.flash_timer = 0.0;
    damage.flash_count = 0;
    damage.crumble_timer = 0.0;
    damage.crumble_index = 0;
    // 记录要移除的尾巴（2节）
    if snake.len() > 3 {
        damage.tail_to_remove = vec![
            snake[snake.len() - 1],
            snake[snake.len() - 2],
        ];
    } else {
        damage.tail_to_remove = vec![];
    }
}

/// 更新受伤动画
/// 
/// 每帧调用，处理闪烁和碎裂两个阶段的状态转换。
/// 
/// ## 参数
/// - `damage`: 受伤状态结构体
/// - `snake`: 蛇身位置列表（碎裂时会移除尾部）
/// - `particles`: 粒子列表（碎裂时生成血液粒子）
/// - `blood_stains`: 血迹列表（碎裂时添加血迹）
/// - `game_time`: 当前游戏时间
/// - `dt`: 帧间隔时间
/// - `rng`: 随机数生成器
fn update_damage_animation(
    damage: &mut DamageState,
    snake: &mut Vec<IVec2>,
    particles: &mut Vec<Particle>,
    blood_stains: &mut Vec<BloodStain>,
    game_time: f32,
    dt: f32,
    rng: &mut ThreadRng,
) {
    match damage.phase {
        DamagePhase::Flashing => {
            damage.flash_timer += dt;
            // 每次闪烁周期
            let flash_period = DAMAGE_FLASH_DURATION / DAMAGE_FLASH_COUNT as f32;
            if damage.flash_timer >= flash_period {
                damage.flash_timer = 0.0;
                damage.flash_count += 1;
                
                if damage.flash_count >= DAMAGE_FLASH_COUNT {
                    // 闪烁结束，进入碎裂阶段
                    damage.phase = DamagePhase::Crumbling;
                    damage.crumble_timer = 0.0;
                    damage.crumble_index = 0;
                }
            }
        }
        DamagePhase::Crumbling => {
            damage.crumble_timer += dt;
            
            if damage.crumble_timer >= DAMAGE_CRUMBLE_INTERVAL {
                damage.crumble_timer = 0.0;
                
                // 碎裂一节尾巴
                if damage.crumble_index < damage.tail_to_remove.len() && snake.len() > 3 {
                    let tail_pos = snake.pop().unwrap();
                    
                    // 生成红色碎裂粒子
                    let center = vec2(
                        tail_pos.x as f32 * CELL + CELL / 2.0,
                        tail_pos.y as f32 * CELL + CELL / 2.0
                    );
                    spawn_blood_particles(particles, center, rng);
                    
                    // 留下血迹
                    blood_stains.push(BloodStain {
                        pos: tail_pos,
                        spawn_time: game_time,
                        lifetime: BLOOD_STAIN_LIFETIME,
                        size: rng.gen_range(0.6..1.0),
                        alpha: rng.gen_range(0.5..0.8),
                    });
                    
                    damage.crumble_index += 1;
                }
                
                // 检查是否完成
                if damage.crumble_index >= damage.tail_to_remove.len() {
                    // 动画结束
                    damage.active = false;
                    damage.phase = DamagePhase::None;
                    damage.tail_to_remove.clear();
                }
            }
        }
        DamagePhase::None => {
            damage.active = false;
        }
    }
}

/// 生成红色血液粒子
/// 
/// 在指定位置生成15个红色粒子，模拟血液飞溅效果。
/// 粒子颜色在深红、亮红、暗红之间随机选择。
/// 
/// ## 参数
/// - `particles`: 粒子列表
/// - `pos`: 生成位置（像素坐标）
/// - `rng`: 随机数生成器
fn spawn_blood_particles(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut ThreadRng) {
    let blood_colors = [
        Color { r: 0.8, g: 0.1, b: 0.1, a: 1.0 }, // 深红
        Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 }, // 亮红
        Color { r: 0.6, g: 0.0, b: 0.0, a: 1.0 }, // 暗红
    ];
    
    for _ in 0..15 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(30.0..120.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let color = blood_colors[rng.gen_range(0..blood_colors.len())];
        let lifetime = rng.gen_range(0.4..0.8);
        
        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(3.0..7.0),
        });
    }
}

/// 更新血迹（淡出消失）
/// 
/// 移除已超过生命周期的血迹。
/// 血迹的淡出效果在绘制时通过alpha计算实现。
fn update_blood_stains(blood_stains: &mut Vec<BloodStain>, game_time: f32) {
    blood_stains.retain(|stain| {
        game_time - stain.spawn_time < stain.lifetime
    });
}

/// 绘制血迹
/// 
/// 绘制所有血迹，包括主血迹和随机溅射的小血滴。
/// 血迹透明度随时间线性衰减，产生淡出效果。
/// 
/// ## 视觉效果
/// - 主血迹：暗红色方块，大小随机（0.6-1.0倍格子大小）
/// - 溅射血滴：更小的暗红色方块，位置使用伪随机生成
fn draw_blood_stains(blood_stains: &[BloodStain], game_time: f32) {
    for stain in blood_stains {
        let elapsed = game_time - stain.spawn_time;
        let fade = 1.0 - (elapsed / stain.lifetime).clamp(0.0, 1.0);
        
        let x = stain.pos.x as f32 * CELL;
        let y = stain.pos.y as f32 * CELL;
        let size = CELL * stain.size;
        let offset = (CELL - size) / 2.0;
        
        // 主血迹
        let color = Color {
            r: 0.5,
            g: 0.0,
            b: 0.0,
            a: stain.alpha * fade,
        };
        draw_rectangle(x + offset, y + offset, size, size, color);
        
        // 添加一些小血滴
        let splatter_color = Color {
            r: 0.4,
            g: 0.0,
            b: 0.0,
            a: stain.alpha * fade * 0.7,
        };
        let splatter_size = size * 0.3;
        // 使用位置生成伪随机溅射
        let hash1 = ((stain.pos.x * 7919 + stain.pos.y * 104729) % 100) as f32 / 100.0;
        let hash2 = ((stain.pos.x * 104729 + stain.pos.y * 7919) % 100) as f32 / 100.0;
        draw_rectangle(
            x + hash1 * (CELL - splatter_size),
            y + hash2 * (CELL - splatter_size),
            splatter_size,
            splatter_size,
            splatter_color
        );
    }
}

// ============================================================================
// 传送门系统
// ============================================================================
// 
// 传送门是游戏中的特殊机制，成对出现，蛇进入一个会从另一个出来。
// 
// ## 生成规则
// - 每15秒有30%概率生成一对传送门
// - 场上同时只能存在一对传送门
// - 两个传送门位置不重叠
// 
// ## 存在时间
// - 传送门存在20秒后消失
// - 最后3秒开始闪烁提示即将消失
// 
// ## 视觉效果
// - 彩色脉动光环（紫色/青绿/橙色随机）
// - 两个传送门之间有半透明连接线
// - 中心有白色闪烁点
// ============================================================================

/// 传送门存在时间（秒）
const PORTAL_LIFETIME: f32 = 20.0;

/// 生成传送门（成对出现）
/// 
/// 在场上生成一对传送门，位置不与蛇身和果实重叠。
/// 
/// ## 返回
/// - `Some(Portal)`: 成功生成的传送门
/// - `None`: 无法找到两个空闲位置
fn spawn_portal(snake: &[IVec2], fruits: &[Fruit], game_time: f32, rng: &mut ThreadRng) -> Option<Portal> {
    // 生成两个不重叠的位置
    let pos_a = spawn_position(snake, fruits, rng)?;
    
    // 临时创建一个假果实来避免第二个位置与第一个重叠
    let temp_fruits: Vec<Fruit> = fruits.iter().cloned()
        .chain(std::iter::once(Fruit {
            pos: pos_a,
            fruit_type: FruitType::Normal,
            spawn_time: 0.0,
            lifetime: 0.0,
        }))
        .collect();
    
    let pos_b = spawn_position(snake, &temp_fruits, rng)?;
    
    // 随机颜色
    let colors = [
        Color { r: 0.5, g: 0.0, b: 1.0, a: 1.0 }, // 紫色
        Color { r: 0.0, g: 1.0, b: 0.5, a: 1.0 }, // 青绿
        Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }, // 橙色
    ];
    let color = colors[rng.gen_range(0..colors.len())];
    
    Some(Portal {
        pos_a,
        pos_b,
        color,
        spawn_time: game_time,
        lifetime: PORTAL_LIFETIME,
    })
}

/// 更新传送门
/// 
/// 移除已超过生命周期的传送门。
fn update_portals(portals: &mut Vec<Portal>, game_time: f32) {
    portals.retain(|p| game_time - p.spawn_time < p.lifetime);
}

/// 绘制传送门
/// 
/// 绘制传送门的视觉效果，包括：
/// - 外圈：传送门颜色的半透明方块
/// - 内圈：脉动的亮色方块
/// - 中心：白色闪烁点
/// - 连接线：两个传送门之间的半透明线
/// 
/// 最后3秒会开始闪烁提示即将消失。
fn draw_portals(portals: &[Portal], game_time: f32) {
    for portal in portals {
        let elapsed = game_time - portal.spawn_time;
        let fade = if elapsed > portal.lifetime - 3.0 {
            // 最后3秒开始闪烁
            let blink = (game_time * 8.0).sin() * 0.5 + 0.5;
            blink * (portal.lifetime - elapsed) / 3.0
        } else {
            1.0
        };
        
        // 绘制两个传送门
        for pos in [portal.pos_a, portal.pos_b] {
            let x = pos.x as f32 * CELL;
            let y = pos.y as f32 * CELL;
            
            // 旋转动画
            let _rotation = game_time * 3.0;
            let pulse = (game_time * 4.0).sin() * 0.15 + 0.85;
            
            // 外圈
            let outer_color = Color {
                r: portal.color.r,
                g: portal.color.g,
                b: portal.color.b,
                a: 0.6 * fade,
            };
            draw_rectangle(x - 2.0, y - 2.0, CELL + 4.0, CELL + 4.0, outer_color);
            
            // 内圈（脉动）
            let inner_size = CELL * pulse;
            let inner_offset = (CELL - inner_size) / 2.0;
            let inner_color = Color {
                r: portal.color.r * 0.5 + 0.5,
                g: portal.color.g * 0.5 + 0.5,
                b: portal.color.b * 0.5 + 0.5,
                a: 0.8 * fade,
            };
            draw_rectangle(x + inner_offset, y + inner_offset, inner_size, inner_size, inner_color);
            
            // 中心点
            let center_size = 6.0;
            let cx = x + CELL / 2.0 - center_size / 2.0;
            let cy = y + CELL / 2.0 - center_size / 2.0;
            draw_rectangle(cx, cy, center_size, center_size, Color { a: fade, ..WHITE });
        }
        
        // 绘制连接线（虚线效果）
        let ax = portal.pos_a.x as f32 * CELL + CELL / 2.0;
        let ay = portal.pos_a.y as f32 * CELL + CELL / 2.0;
        let bx = portal.pos_b.x as f32 * CELL + CELL / 2.0;
        let by = portal.pos_b.y as f32 * CELL + CELL / 2.0;
        let line_color = Color { a: 0.3 * fade, ..portal.color };
        draw_line(ax, ay, bx, by, 1.0, line_color);
    }
}

// ============================================================================
// Combo系统
// ============================================================================
// 
// Combo系统追踪连续吃果实的连击数，提供额外分数奖励。
// 
// ## 连击规则
// - 1.5秒内连续吃果实会累积combo
// - combo数越高，每个果实额外加分越多
// - 额外加分 = min(combo数, 5)，即最高+5分
// - 吃到陷阱果实会重置combo
// 
// ## 显示效果
// - combo>1时在屏幕右上角显示"COMBO x数字!"
// - 颜色随combo数变化：2=黄色, 3=橙色, 4=深橙, 5+=红色
// - 字体大小随combo数增加（最大+10像素）
// ============================================================================

/// Combo时间窗口（秒）
/// 
/// 在此时间内连续吃果实才算连击
const COMBO_WINDOW: f32 = 1.5;

/// 更新Combo
/// 
/// 检查距离上次吃果实的时间，决定是累加还是重置combo。
/// 
/// ## 参数
/// - `combo`: Combo状态结构体
/// - `game_time`: 当前游戏时间
fn update_combo(combo: &mut ComboState, game_time: f32) {
    let time_since_last = game_time - combo.last_eat_time;
    
    if time_since_last <= COMBO_WINDOW {
        combo.count += 1;
    } else {
        combo.count = 1;
    }
    
    combo.last_eat_time = game_time;
    combo.display_timer = 1.5; // 显示1.5秒
}

// ============================================================================
// 残影系统（速度模式）
// ============================================================================
// 
// 速度模式下蛇移动时留下的蓝色残影效果。
// 
// ## 实现原理
// - 每次移动时记录当前蛇身位置快照
// - 残影随时间淡出（0.3秒）
// - 颜色为半透明蓝色，与速度模式的蛇身颜色呼应
// 
// ## 视觉效果
// - 多个残影叠加产生拖尾效果
// - 透明度随时间线性衰减
// - 增强速度感和动态感
// ============================================================================

/// 残影存在时间（秒）
const AFTERIMAGE_LIFETIME: f32 = 0.3;

/// 更新残影
/// 
/// 移除已超过生命周期的残影。
fn update_afterimages(afterimages: &mut Vec<Afterimage>, game_time: f32) {
    afterimages.retain(|a| game_time - a.spawn_time < AFTERIMAGE_LIFETIME);
}

/// 绘制残影
/// 
/// 绘制所有残影，透明度随时间衰减。
/// 颜色为半透明蓝色（与速度模式蛇身颜色一致）。
fn draw_afterimages(afterimages: &[Afterimage], game_time: f32) {
    for afterimage in afterimages {
        let elapsed = game_time - afterimage.spawn_time;
        let fade = 1.0 - (elapsed / AFTERIMAGE_LIFETIME);
        let alpha = afterimage.alpha * fade;
        
        let color = Color {
            r: 0.2,
            g: 0.5,
            b: 1.0,
            a: alpha * 0.5,
        };
        
        for seg in &afterimage.positions {
            let x = seg.x as f32 * CELL;
            let y = seg.y as f32 * CELL;
            draw_rectangle(x, y, CELL, CELL, color);
        }
    }
}

// ============================================================================
// 幽灵效果
// ============================================================================
// 
// 幽灵模式下蛇身周围的白色脉动光环效果。
// 
// ## 视觉效果
// - 白色半透明光环包围蛇身
// - 光环大小比蛇身大6像素（每边3像素）
// - 透明度随时间脉动（0.14-0.2之间）
// 
// ## 功能提示
// 光环效果提示玩家当前处于幽灵模式，可以穿墙和穿过自己。
// ============================================================================

/// 绘制幽灵效果（白色光环）
/// 
/// 在蛇身周围绘制脉动的白色半透明光环。
fn draw_ghost_effect(snake: &[IVec2], time: f32) {
    let pulse = (time * 6.0).sin() * 0.3 + 0.7;
    let ghost_color = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.2 * pulse };
    
    for seg in snake {
        let x = seg.x as f32 * CELL - 3.0;
        let y = seg.y as f32 * CELL - 3.0;
        let size = CELL + 6.0;
        draw_rectangle(x, y, size, size, ghost_color);
    }
}

// ============================================================================
// 冰冻效果系统
// ============================================================================
// 
// 冰冻陷阱的视觉效果系统，包括寒气粒子和冰晶覆盖。
// 
// ## 效果组成
// 1. **初始爆发粒子**: 被冰冻瞬间在蛇身周围生成大量寒气粒子
// 2. **持续环境粒子**: 冰冻期间持续生成少量飘动的寒气粒子
// 3. **冰晶覆盖**: 在蛇身上绘制闪烁的冰晶纹理
// 
// ## 粒子颜色
// - 浅蓝、冰蓝、白色、淡青四种颜色随机选择
// - 粒子向上飘动，模拟寒气上升效果
// 
// ## 冰晶纹理
// - 闪烁的边框和内部冰晶点
// - 伪随机生成的冰裂纹
// - 使用位置哈希确保同一位置的纹理一致
// ============================================================================

/// 生成冰冻初始爆发粒子
/// 
/// 当蛇被冰冻时，在蛇身周围生成大量寒气粒子。
/// 每个蛇身节生成5-8个粒子，粒子向上飘动。
/// 
/// ## 参数
/// - `particles`: 粒子列表
/// - `snake`: 蛇身位置列表
/// - `rng`: 随机数生成器
fn spawn_freeze_particles(particles: &mut Vec<Particle>, snake: &[IVec2], rng: &mut ThreadRng) {
    let ice_colors = [
        Color { r: 0.7, g: 0.9, b: 1.0, a: 1.0 }, // 浅蓝
        Color { r: 0.5, g: 0.8, b: 1.0, a: 1.0 }, // 冰蓝
        Color { r: 0.9, g: 0.95, b: 1.0, a: 1.0 }, // 白色
        Color { r: 0.6, g: 0.85, b: 0.95, a: 1.0 }, // 淡青
    ];
    
    // 在每个蛇身节周围生成粒子
    for seg in snake {
        let center = vec2(
            seg.x as f32 * CELL + CELL / 2.0,
            seg.y as f32 * CELL + CELL / 2.0
        );
        
        // 每节生成5-8个粒子
        for _ in 0..rng.gen_range(5..8) {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(20.0..80.0);
            let vel = vec2(angle.cos() * speed, angle.sin() * speed - 30.0); // 向上飘
            let color = ice_colors[rng.gen_range(0..ice_colors.len())];
            let lifetime = rng.gen_range(0.5..1.2);
            
            particles.push(Particle {
                pos: center,
                vel,
                color,
                lifetime,
                max_lifetime: lifetime,
                size: rng.gen_range(2.0..5.0),
            });
        }
    }
}

/// 生成持续的寒气环境粒子
/// 
/// 冰冻期间持续在蛇身周围生成少量飘动的寒气粒子。
/// 每帧有30%概率调用此函数，在随机一个蛇身节位置生成1个粒子。
/// 
/// ## 参数
/// - `particles`: 粒子列表
/// - `snake`: 蛇身位置列表
/// - `rng`: 随机数生成器
fn spawn_frost_ambient_particles(particles: &mut Vec<Particle>, snake: &[IVec2], rng: &mut ThreadRng) {
    let ice_colors = [
        Color { r: 0.8, g: 0.95, b: 1.0, a: 0.8 },
        Color { r: 0.6, g: 0.9, b: 1.0, a: 0.7 },
    ];
    
    // 随机选择一个蛇身节
    if let Some(seg) = snake.get(rng.gen_range(0..snake.len())) {
        let x = seg.x as f32 * CELL + rng.gen_range(0.0..CELL);
        let y = seg.y as f32 * CELL + rng.gen_range(0.0..CELL);
        
        let vel = vec2(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-30.0..-10.0) // 向上飘
        );
        let color = ice_colors[rng.gen_range(0..ice_colors.len())];
        let lifetime = rng.gen_range(0.3..0.8);
        
        particles.push(Particle {
            pos: vec2(x, y),
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(2.0..4.0),
        });
    }
}

/// 绘制冰晶覆盖效果
/// 
/// 在冰冻的蛇身上绘制闪烁的冰晶纹理。
/// 
/// ## 视觉效果
/// - 闪烁的白色边框
/// - 内部随机分布的冰晶点（位置使用伪随机哈希确保一致性）
/// - 对角线冰裂纹（50%概率出现）
/// 
/// ## 参数
/// - `x`, `y`: 格子左上角像素坐标
/// - `time`: 当前游戏时间（用于闪烁动画）
/// - `seg_x`, `seg_y`: 格子网格坐标（用于伪随机哈希）
fn draw_ice_crystal(x: f32, y: f32, time: f32, seg_x: i32, seg_y: i32) {
    // 冰晶闪烁效果
    let sparkle = ((time * 8.0 + (seg_x + seg_y) as f32 * 0.5).sin() * 0.5 + 0.5).powi(2);
    
    // 绘制冰晶边框
    let border_color = Color {
        r: 0.9,
        g: 0.95,
        b: 1.0,
        a: 0.4 + sparkle * 0.3,
    };
    draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);
    
    // 绘制冰晶内部纹理（使用伪随机位置）
    let hash1 = ((seg_x * 7919 + seg_y * 104729) % 100) as f32 / 100.0;
    let hash2 = ((seg_x * 104729 + seg_y * 7919) % 100) as f32 / 100.0;
    let hash3 = ((seg_x * 31 + seg_y * 17) % 100) as f32 / 100.0;
    
    // 冰晶点
    let crystal_color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 0.5 + sparkle * 0.4,
    };
    let crystal_size = 3.0 + sparkle * 2.0;
    
    // 绘制几个闪烁的冰晶点
    draw_rectangle(
        x + hash1 * (CELL - crystal_size),
        y + hash2 * (CELL - crystal_size),
        crystal_size,
        crystal_size,
        crystal_color
    );
    draw_rectangle(
        x + hash2 * (CELL - crystal_size),
        y + hash3 * (CELL - crystal_size),
        crystal_size * 0.7,
        crystal_size * 0.7,
        crystal_color
    );
    
    // 绘制对角线冰裂纹
    if hash1 > 0.5 {
        let crack_color = Color { r: 0.8, g: 0.9, b: 1.0, a: 0.3 };
        draw_line(x + 2.0, y + 2.0, x + CELL - 2.0, y + CELL - 2.0, 1.0, crack_color);
    }
    if hash2 > 0.5 {
        let crack_color = Color { r: 0.8, g: 0.9, b: 1.0, a: 0.3 };
        draw_line(x + CELL - 2.0, y + 2.0, x + 2.0, y + CELL - 2.0, 1.0, crack_color);
    }
}

// ============================================================================
// 幸运方块系统
// ============================================================================
// 
// 幸运方块是特殊果实，外观为黄色礼盒带问号。
// 吃到后随机触发增益或负面效果（各50%概率）。
// 
// ## 增益效果（50%）
// - 护盾（10秒无敌）
// - 速度（5秒加速）
// - 幽灵（5秒穿墙）
// - 额外分数（+10分）
// - 蛇身增长（+3节）
// 
// ## 负面效果（50%，受免疫影响）
// - 冰冻（2秒）
// - 减速（5秒）
// - 眩晕（5秒）
// - 粘液（5秒）
// - 蛇身减少（-2节）
// ============================================================================

/// 绘制幸运方块果实
/// 
/// 绘制黄色礼盒外观，中间有脉动的问号。
/// 
/// ## 视觉效果
/// - 黄色底色礼盒
/// - 红色丝带（十字形）
/// - 白色脉动问号
/// - 轻微的闪烁效果
fn draw_lucky_fruit(x: f32, y: f32, time: f32) {
    // 礼盒底色（黄色）
    let box_color = Color { r: 1.0, g: 0.85, b: 0.2, a: 1.0 };
    draw_rectangle(x, y, CELL, CELL, box_color);
    
    // 礼盒边框（深黄色）
    let border_color = Color { r: 0.8, g: 0.6, b: 0.1, a: 1.0 };
    draw_rectangle_lines(x, y, CELL, CELL, 2.0, border_color);
    
    // 红色丝带（十字形）
    let ribbon_color = Color { r: 0.9, g: 0.2, b: 0.2, a: 1.0 };
    let ribbon_width = 4.0;
    // 水平丝带
    draw_rectangle(x, y + CELL / 2.0 - ribbon_width / 2.0, CELL, ribbon_width, ribbon_color);
    // 垂直丝带
    draw_rectangle(x + CELL / 2.0 - ribbon_width / 2.0, y, ribbon_width, CELL, ribbon_color);
    
    // 问号（脉动效果）
    let pulse = (time * 4.0).sin() * 0.2 + 0.8;
    let question_color = Color { r: 1.0, g: 1.0, b: 1.0, a: pulse };
    
    // 问号的圆弧部分（简化为几个方块）
    let qx = x + CELL / 2.0;
    let qy = y + CELL / 2.0;
    let qs = 3.0; // 问号笔画大小
    
    // 问号顶部弧线（用方块近似）
    draw_rectangle(qx - qs, qy - 6.0, qs * 2.0, qs, question_color);
    draw_rectangle(qx + qs - 1.0, qy - 5.0, qs, qs * 2.0, question_color);
    draw_rectangle(qx - 1.0, qy - 1.0, qs, qs, question_color);
    
    // 问号底部的点
    draw_rectangle(qx - qs / 2.0, qy + 4.0, qs, qs, question_color);
}

/// 生成幸运方块开箱粒子效果
/// 
/// 在幸运方块被吃掉时生成彩色粒子爆发效果。
/// 粒子颜色为彩虹色，模拟开箱的惊喜感。
fn spawn_lucky_particles(particles: &mut Vec<Particle>, pos: Vec2, rng: &mut ThreadRng) {
    let colors = [
        Color { r: 1.0, g: 0.2, b: 0.2, a: 1.0 }, // 红
        Color { r: 1.0, g: 0.6, b: 0.2, a: 1.0 }, // 橙
        Color { r: 1.0, g: 1.0, b: 0.2, a: 1.0 }, // 黄
        Color { r: 0.2, g: 1.0, b: 0.2, a: 1.0 }, // 绿
        Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 }, // 蓝
        Color { r: 0.8, g: 0.2, b: 1.0, a: 1.0 }, // 紫
    ];
    
    // 生成彩色粒子爆发
    for _ in 0..25 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(80.0..200.0);
        let vel = vec2(angle.cos() * speed, angle.sin() * speed);
        let color = colors[rng.gen_range(0..colors.len())];
        let lifetime = rng.gen_range(0.5..1.0);
        
        particles.push(Particle {
            pos,
            vel,
            color,
            lifetime,
            max_lifetime: lifetime,
            size: rng.gen_range(3.0..8.0),
        });
    }
}
