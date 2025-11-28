#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
//! # Snake 2D V2 - 模块化贪吃蛇游戏
//!
//! 使用模块化架构重构的贪吃蛇游戏主入口。
//!
//! ## 架构说明
//!
//! 本文件只包含：
//! - 游戏世界状态结构体 (`GameWorld`)
//! - 窗口配置 (`window_conf`)
//! - 游戏主循环 (`main`)
//!
//! 所有游戏逻辑、渲染、音效都封装在独立模块中。

// =============================================================================
// 外部依赖
// =============================================================================

use macroquad::prelude::*;
// macroquad::prelude 包含:
// - IVec2, Vec2: 向量类型
// - ivec2, vec2: 向量构造函数
// - Color, RED, GREEN, LIME 等: 颜色
// - draw_rectangle, draw_line 等: 绘图函数
// - is_key_pressed, KeyCode: 输入处理
// - get_frame_time, next_frame: 帧控制
// - Conf: 窗口配置

use ::rand::{thread_rng, Rng};
// thread_rng: 获取线程本地随机数生成器
// Rng: 随机数生成器 trait，提供 gen_bool, gen_range 等方法

// =============================================================================
// 游戏库模块导入
// =============================================================================

// --- 常量模块 ---
use rtest::constants::{
    CELL,    // 单元格像素尺寸 (20.0)
    GRID_W,  // 游戏区域宽度格子数 (32)
    GRID_H,  // 游戏区域高度格子数 (24)
};

// --- 类型模块 ---
use rtest::types::{
    // 游戏状态
    GameState,      // 游戏状态枚举: Playing, Paused, GameOver
    BuffState,      // Buff状态管理: 护盾、速度、幽灵、冰冻等
    DamageState,    // 受伤动画状态
    ComboState,     // Combo连击状态
    SandwormPhase,  // 沙虫变身阶段枚举

    // 游戏对象
    Fruit,          // 果实实例 (位置、类型、生命周期)
    Portal,         // 传送门 (两个位置、颜色、生命周期)
    Particle,       // 粒子 (位置、速度、颜色、生命周期)
    BloodStain,     // 血迹 (位置、透明度、生命周期)
    Afterimage,     // 残影 (蛇身位置快照)
};

// --- 果实系统模块 ---
use rtest::fruits::{
    create_fruit_registry,  // 创建并初始化果实注册表
    FruitRegistry,          // 果实注册表: 管理所有果实类型
};

// --- 游戏逻辑模块 ---
use rtest::game::{
    // 蛇
    Snake,              // 蛇结构体: 身体位置、方向、移动
    MoveResult,         // 移动结果: Normal, WallCollision, SelfCollision

    // 碰撞检测
    check_fruit_collision,   // 检查蛇头是否碰到果实
    check_portal_collision,  // 检查蛇头是否碰到传送门

    // 生成逻辑
    spawn_food,         // 生成食物位置
    spawn_portal,       // 生成传送门
    update_fruits_with_callbacks, // 更新果实列表并调用过期回调
    update_portals,     // 更新传送门列表 (移除过期)

    // 状态更新
    update_combo,               // 更新Combo状态
    update_particles,           // 更新粒子 (移动、衰减)
    update_damage_animation,    // 更新受伤动画状态机
    update_blood_stains,        // 更新血迹 (移除过期)
    spawn_blood_particles,      // 生成血液粒子
    update_sandworm_mode,       // 更新沙虫模式状态机

    // 果实效果
    handle_fruit_effect,        // 处理果实消费效果

    // AI 蛇系统
    AIManager,          // AI蛇管理器: 生成、更新、碰撞检测
    
    // 果实生成管理器
    FruitSpawnManager,          // 果实生成管理器
    create_default_spawn_manager, // 创建默认生成管理器
    
    // 炸弹管理器
    BombManager,        // 炸弹管理器
};

// --- 渲染模块 ---
use rtest::render::{
    // 基础渲染
    draw_background,        // 绘制背景 (清屏)
    draw_border_and_grid,   // 绘制边框和网格线

    // 游戏对象渲染
    draw_food,              // 绘制食物
    draw_fruits,            // 绘制所有果实
    draw_snake,             // 绘制蛇 (含各种Buff视觉效果)
    draw_sandworm_mode,     // 绘制沙虫模式 (特殊渲染)

    // 特效渲染
    draw_particles,         // 绘制粒子
    draw_blood_stains,      // 绘制血迹
    draw_portals,           // 绘制传送门
    draw_afterimages,       // 绘制残影
    draw_shield_effect,     // 绘制护盾光环
    draw_ghost_effect,      // 绘制幽灵效果
    spawn_freeze_particles, // 生成冰冻粒子

    // AI蛇渲染
    draw_ai_snakes,         // 绘制所有AI蛇
    draw_dropped_foods,     // 绘制掉落的食物

    // UI渲染
    draw_hud,               // 绘制HUD (分数、Combo、Buff状态)
    draw_overlay,           // 绘制覆盖层 (暂停、游戏结束)
};

// --- 音效模块 ---
use rtest::audio::SoundManager;
// SoundManager: 音效管理器
// - new(): 异步创建并加载所有音效
// - play_eat(): 播放吃食物音效
// - play_trap(): 播放陷阱音效
// - play_power(): 播放功能果实音效
// - play_game_over(): 播放游戏结束音效

// =============================================================================
// 游戏世界状态
// =============================================================================

/// 游戏世界状态
///
/// 集中管理所有游戏状态，作为游戏主循环的核心数据结构。
///
/// ## 设计说明
///
/// 将所有状态集中在一个结构体中的好处：
/// - 便于序列化/反序列化（存档功能）
/// - 便于重置游戏状态
/// - 清晰的所有权边界
struct GameWorld {
    // -------------------------------------------------------------------------
    // 核心游戏对象
    // -------------------------------------------------------------------------

    /// 蛇实例
    ///
    /// 包含蛇身位置列表、移动方向、上一帧位置（用于插值）
    snake: Snake,

    /// 食物位置
    ///
    /// 普通食物，吃到后+1分并增长1节
    food: IVec2,

    /// 果实列表
    ///
    /// 特殊果实（陷阱、功能、幸运方块等），有生命周期
    fruits: Vec<Fruit>,

    /// 传送门列表
    ///
    /// 成对出现，碰到一个会传送到另一个
    portals: Vec<Portal>,

    // -------------------------------------------------------------------------
    // 效果状态
    // -------------------------------------------------------------------------

    /// Buff状态
    ///
    /// 管理所有增益/减益效果：护盾、速度、幽灵、冰冻、减速、眩晕、粘液、沙虫
    buff_state: BuffState,

    /// 受伤状态
    ///
    /// 管理受伤动画：闪烁 → 逐节掉落
    damage_state: DamageState,

    /// Combo状态
    ///
    /// 连续吃果实的计数和显示计时器
    combo_state: ComboState,

    // -------------------------------------------------------------------------
    // 视觉效果
    // -------------------------------------------------------------------------

    /// 粒子列表
    ///
    /// 用于爆炸、血液飞溅等特效
    particles: Vec<Particle>,

    /// 血迹列表
    ///
    /// 受伤时留下的血迹，会逐渐消失
    blood_stains: Vec<BloodStain>,

    /// 残影列表
    ///
    /// 速度模式下的蛇身残影
    afterimages: Vec<Afterimage>,

    // -------------------------------------------------------------------------
    // 游戏控制
    // -------------------------------------------------------------------------

    /// 游戏状态
    ///
    /// Playing: 游戏进行中
    /// Paused: 暂停
    /// GameOver: 游戏结束
    state: GameState,

    /// 当前分数
    score: u32,

    /// 最高分
    high_score: u32,

    /// 游戏时间（秒）
    ///
    /// 用于动画和果实生命周期计算
    game_time: f32,

    /// 是否启用穿墙模式
    ///
    /// true: 蛇可以从一边穿到另一边
    /// false: 撞墙游戏结束
    wrap: bool,

    // -------------------------------------------------------------------------
    // 生成计时器
    // -------------------------------------------------------------------------

    /// 传送门生成计时器
    portal_spawn_timer: f32,

    // -------------------------------------------------------------------------
    // 系统
    // -------------------------------------------------------------------------

    /// 果实注册表
    ///
    /// 管理所有已注册的果实类型
    registry: FruitRegistry,
    
    /// 果实生成管理器
    ///
    /// 统一管理所有果实类别的生成逻辑，支持声明式配置
    spawn_manager: FruitSpawnManager,

    // -------------------------------------------------------------------------
    // AI 蛇系统
    // -------------------------------------------------------------------------

    /// AI 蛇管理器
    ///
    /// 管理所有 AI 蛇的生成、移动、碰撞和死亡
    ai_manager: AIManager,
}

impl GameWorld {
    /// 创建新的游戏世界
    fn new() -> Self {
        let mut rng = thread_rng();
        let snake = Snake::new();
        let food = spawn_food(&snake.body, &mut rng);

        GameWorld {
            snake,
            food,
            fruits: vec![],
            portals: vec![],
            buff_state: BuffState::default(),
            damage_state: DamageState::default(),
            combo_state: ComboState::default(),
            particles: vec![],
            blood_stains: vec![],
            afterimages: vec![],
            state: GameState::Playing,
            score: 0,
            high_score: 0,
            game_time: 0.0,
            wrap: true,
            portal_spawn_timer: 0.0,
            registry: create_fruit_registry(),
            spawn_manager: create_default_spawn_manager(),
            ai_manager: AIManager::new(),
        }
    }

    /// 重置游戏状态
    ///
    /// 保留最高分，重置其他所有状态
    fn reset(&mut self) {
        let mut rng = thread_rng();
        self.snake.reset();
        self.food = spawn_food(&self.snake.body, &mut rng);
        self.fruits.clear();
        self.portals.clear();
        self.buff_state = BuffState::default();
        self.damage_state = DamageState::default();
        self.combo_state = ComboState::default();
        self.particles.clear();
        self.blood_stains.clear();
        self.afterimages.clear();
        self.state = GameState::Playing;
        self.score = 0;
        self.game_time = 0.0;
        self.portal_spawn_timer = 0.0;
        self.spawn_manager.reset();
        self.ai_manager.reset();
    }
}

// =============================================================================
// 窗口配置
// =============================================================================

/// Macroquad 窗口配置
///
/// 设置窗口标题、尺寸等属性
fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake 2D V2"),
        window_width: (GRID_W as f32 * CELL) as i32,  // 32 * 20 = 640
        window_height: (GRID_H as f32 * CELL) as i32, // 24 * 20 = 480
        fullscreen: false,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

// =============================================================================
// 游戏主循环
// =============================================================================

#[macroquad::main(window_conf)]
async fn main() {
    // -------------------------------------------------------------------------
    // 初始化
    // -------------------------------------------------------------------------

    let mut world = GameWorld::new();
    let mut rng = thread_rng();
    let sounds = SoundManager::new().await;
    let mut over_once = false;      // 防止重复播放游戏结束音效
    let mut accumulator = 0.0f32;   // 固定时间步长累加器

    // -------------------------------------------------------------------------
    // 主循环
    // -------------------------------------------------------------------------

    loop {
        // =====================================================================
        // 计算 Tick 间隔
        // =====================================================================
        // 蛇越长移动越快，Buff 会影响速度

        let len = world.snake.len() as u32;
        // 基础 tick: 120ms，每增长3节减少5ms，最小40ms
        let mut tick = ((120u32.saturating_sub((len / 3) * 5)).max(40)) as f32 / 1000.0;
        // 应用 Buff 倍率（速度模式 0.5x，减速 2x，粘液 1.5x）
        tick *= world.buff_state.tick_multiplier();
        accumulator += get_frame_time();

        // =====================================================================
        // 输入处理
        // =====================================================================

        // --- 退出游戏 ---
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        // --- 暂停/继续 ---
        if is_key_pressed(KeyCode::Space) {
            world.state = match world.state {
                GameState::Playing => GameState::Paused,
                GameState::Paused => GameState::Playing,
                s => s,
            };
        }

        // --- 重新开始 ---
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::R) {
            if world.state != GameState::Playing {
                world.reset();
                over_once = false;
            }
        }

        // --- 切换穿墙模式 ---
        if is_key_pressed(KeyCode::W) {
            world.wrap = !world.wrap;
        }

        // --- 方向输入 ---
        // 使用 is_key_pressed 只检测按键首次按下，忽略按住时的重复事件
        // 这样可以避免按住方向键时的卡顿问题
        if world.state == GameState::Playing && !world.buff_state.frozen {
            // 获取本帧新按下的方向（只响应首次按下，不响应按住）
            let pressed_dir = if is_key_pressed(KeyCode::Up) {
                Some(ivec2(0, -1))
            } else if is_key_pressed(KeyCode::Down) {
                Some(ivec2(0, 1))
            } else if is_key_pressed(KeyCode::Left) {
                Some(ivec2(-1, 0))
            } else if is_key_pressed(KeyCode::Right) {
                Some(ivec2(1, 0))
            } else {
                None
            };

            // 只有当有新按键且不是反向时才更新方向
            if let Some(new_dir) = pressed_dir {
                // 防止反向移动（不能直接掉头）
                let is_opposite = new_dir.x == -world.snake.dir.x && new_dir.y == -world.snake.dir.y
                    && (world.snake.dir.x != 0 || world.snake.dir.y != 0);
                
                if !is_opposite {
                    let mut final_dir = new_dir;
                    
                    // 眩晕效果：40% 概率方向偏移
                    if world.buff_state.dizzy_active {
                        if rng.gen_bool(0.4) {
                            let directions = [ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0), ivec2(1, 0)];
                            let valid_dirs: Vec<_> = directions
                                .iter()
                                .filter(|&&d| d != ivec2(-world.snake.dir.x, -world.snake.dir.y))
                                .collect();
                            if !valid_dirs.is_empty() {
                                final_dir = *valid_dirs[rng.gen_range(0..valid_dirs.len())];
                            }
                        }
                    }
                    world.snake.dir = final_dir;
                }
            }
        }

        // =====================================================================
        // 固定时间步长游戏逻辑
        // =====================================================================
        // 每个 tick 蛇移动一格，最多处理5步防止卡顿时跳帧过多

        let mut steps = 0;
        while accumulator >= tick && steps < 5 {
            accumulator -= tick;

            // 非游戏中状态跳过
            if world.state != GameState::Playing {
                break;
            }

            // 冰冻状态跳过移动
            if world.buff_state.frozen {
                continue;
            }

            steps += 1;

            // --- 速度模式残影 ---
            if world.buff_state.speed_active {
                world.afterimages.push(Afterimage {
                    positions: world.snake.body.clone(),
                    alpha: 0.4,
                    spawn_time: world.game_time,
                });
            }

            // --- 沙虫模式跳过正常移动 ---
            if world.buff_state.sandworm_phase != SandwormPhase::None {
                continue;
            }

            // --- 移动蛇 ---
            // 护盾/幽灵/沙虫模式可穿过自己
            let can_pass_self = world.buff_state.can_pass_through();
            let move_result = world.snake.move_forward(world.wrap, can_pass_self);

            match move_result {
                // 碰撞处理
                MoveResult::WallCollision | MoveResult::SelfCollision => {
                    if !world.buff_state.can_pass_through() {
                        world.state = GameState::GameOver;
                        break;
                    }
                }
                // 正常移动
                MoveResult::Normal(mut new_head) => {
                    // 检查传送门 - 只传送蛇头，身体会自然跟随穿过
                    if let Some((_from, to)) = check_portal_collision(new_head, &world.portals) {
                        // 将蛇头移动到传送门出口
                        world.snake.body[0] = to;
                        new_head = to;
                    }

                    // 检查食物
                    if new_head == world.food {
                        world.score += 1;
                        world.snake.grow();
                        world.food = spawn_food(&world.snake.body, &mut rng);
                        sounds.play_eat();
                    }

                    // 检查果实
                    if let Some(idx) = check_fruit_collision(new_head, &world.fruits) {
                        let fruit = world.fruits.remove(idx);
                        
                        // 蛇蛋被吃掉 = 阻止孵化，不生成 AI 蛇
                        // AI 蛇只在蛇蛋过期时自动生成
                        
                        update_combo(&mut world.combo_state, world.game_time);

                        handle_fruit_effect(
                            &fruit,
                            &world.registry,
                            &mut world.snake,
                            &mut world.buff_state,
                            &mut world.damage_state,
                            &mut world.particles,
                            &mut world.score,
                            &mut world.combo_state,
                            &mut world.state,
                            &mut rng,
                            world.game_time,
                            &sounds,
                        );
                    }
                    
                    // 检查是否吃到 AI 蛇掉落的食物
                    let eaten = world.ai_manager.check_player_eat_dropped(new_head);
                    if eaten > 0 {
                        world.score += eaten;
                        world.snake.grow();
                        sounds.play_eat();
                    }
                    
                    // 检查是否撞到 AI 蛇身体
                    if !world.buff_state.can_pass_through() {
                        for ai_snake in &world.ai_manager.snakes {
                            // 撞到 AI 蛇身体（不包括头）
                            if ai_snake.body.iter().skip(1).any(|&p| p == new_head) {
                                world.state = GameState::GameOver;
                                break;
                            }
                            // 头对头碰撞
                            if ai_snake.head() == new_head {
                                world.state = GameState::GameOver;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // =====================================================================
        // 更新游戏系统
        // =====================================================================

        let dt = get_frame_time();

        if world.state == GameState::Playing {
            world.game_time += dt;

            // --- 更新沙虫模式 ---
            let sandworm_result = update_sandworm_mode(
                &mut world.snake,
                &mut world.buff_state,
                &mut world.fruits,
                &mut world.food,
                &mut world.particles,
                dt,
                &mut rng,
            );
            world.score += sandworm_result.bonus_score;
            
            // 沙虫模式吞噬 AI 蛇
            if world.buff_state.sandworm_phase != SandwormPhase::None {
                let devour_score = world.ai_manager.sandworm_devour(
                    &world.snake.body,
                    &mut world.particles,
                    &mut rng,
                );
                world.score += devour_score;
            }

            // --- 更新 Buff 计时器 ---
            world.buff_state.update(dt);
            
            // --- 更新炸弹状态（使用 BombManager） ---
            let bomb_result = BombManager::update(
                &mut world.buff_state,
                &world.snake.body,
                dt,
                &mut rng,
            );
            
            // 处理炸弹爆炸结果
            if let Some(truncate_pos) = bomb_result.truncate_to {
                // 添加爆炸粒子
                world.particles.extend(bomb_result.particles);
                // 截断蛇身
                world.snake.body.truncate(truncate_pos);
                // 检查游戏结束
                if bomb_result.game_over || world.snake.body.len() < 3 {
                    world.state = GameState::GameOver;
                }
            }
            
            // --- 炸弹后遗症掉血（使用 BombManager） ---
            let (need_bleed, bleed_game_over) = BombManager::update_after_effect(
                &mut world.buff_state,
                world.snake.body.len(),
                dt,
            );
            if need_bleed {
                // 获取尾部位置用于生成血迹
                if let Some(&tail_pos) = world.snake.body.last() {
                    let center = vec2(
                        tail_pos.x as f32 * CELL + CELL / 2.0,
                        tail_pos.y as f32 * CELL + CELL / 2.0,
                    );
                    // 生成血液粒子
                    spawn_blood_particles(&mut world.particles, center, &mut rng);
                    // 留下血迹
                    world.blood_stains.push(BloodStain {
                        pos: tail_pos,
                        spawn_time: world.game_time,
                        lifetime: 5.0,
                        size: rng.gen_range(0.6..1.0),
                        alpha: rng.gen_range(0.5..0.8),
                    });
                }
                world.snake.body.pop();
            }
            if bleed_game_over {
                world.state = GameState::GameOver;
            }

            // --- 冰冻粒子效果 ---
            if world.buff_state.frozen && rng.gen_bool(0.3) {
                spawn_freeze_particles(&mut world.particles, &world.snake.body, &mut rng);
            }
            
            // --- 更新 AI 蛇决策 ---
            world.ai_manager.update_thinking(
                world.food,
                &world.fruits,
                &world.snake.body,
                world.wrap,
                dt,
                &mut rng,
            );
            
            // --- 更新 AI 蛇移动 ---
            let ai_result = world.ai_manager.update_movement(
                &mut world.food,
                &mut world.fruits,
                &world.snake.body,
                world.buff_state.can_pass_through(),
                world.buff_state.ghost_active,  // 幽灵状态：AI蛇可以穿过玩家
                &mut world.particles,
                &world.registry,
                world.wrap,
                world.game_time,
                dt,
                &mut rng,
            );
            
            // AI 蛇撞到玩家导致玩家死亡
            if ai_result.player_died {
                world.state = GameState::GameOver;
            }
            
            // --- 更新掉落的食物 ---
            world.ai_manager.update_dropped_foods(world.game_time);

            // --- 更新果实（移除过期并调用 on_expire 回调） ---
            // 所有过期逻辑都在各果实的 on_expire 回调中处理
            // 例如：蛇蛋过期时会在其 on_expire 中直接调用 ai_manager.spawn_snake()
            let _expired_fruits = update_fruits_with_callbacks(
                &mut world.fruits,
                &world.registry,
                &mut world.snake.body,
                &mut world.snake.dir,
                &mut world.buff_state,
                &mut world.damage_state,
                &mut world.particles,
                &mut world.score,
                &mut world.combo_state,
                &mut world.ai_manager,
                &mut world.food,
                world.game_time,
                &mut rng,
            );

            // --- 果实生成（使用 FruitSpawnManager 统一管理） ---
            // 所有生成规则都在 create_default_spawn_manager() 中声明式配置
            world.spawn_manager.update(
                &world.registry,
                &world.snake.body,
                &mut world.fruits,
                world.game_time,
                world.buff_state.sandworm_active,
                dt,
                &mut rng,
            );

            // --- 传送门生成 ---
            world.portal_spawn_timer += dt;
            if world.portal_spawn_timer >= 15.0 && world.portals.is_empty() {
                world.portal_spawn_timer = 0.0;
                if rng.gen_bool(0.3) {
                    if let Some(portal) = spawn_portal(
                        &world.snake.body,
                        &world.fruits,
                        world.game_time,
                        &mut rng,
                    ) {
                        world.portals.push(portal);
                    }
                }
            }

            // --- 更新传送门和残影 ---
            update_portals(&mut world.portals, world.game_time);
            world
                .afterimages
                .retain(|a| world.game_time - a.spawn_time < 0.3);

            // --- 更新 Combo 显示 ---
            world.combo_state.display_timer -= dt;
        }

        // --- 更新粒子 ---
        update_particles(&mut world.particles, dt);

        // --- 更新受伤动画 ---
        if world.damage_state.active {
            update_damage_animation(
                &mut world.damage_state,
                &mut world.snake.body,
                &mut world.particles,
                &mut world.blood_stains,
                world.game_time,
                dt,
                &mut rng,
            );
        }

        // --- 更新血迹 ---
        update_blood_stains(&mut world.blood_stains, world.game_time);

        // =====================================================================
        // 渲染
        // =====================================================================
        // 渲染顺序很重要，后绘制的会覆盖先绘制的

        // --- 背景层 ---
        draw_background();
        draw_border_and_grid();

        // --- 地面层 ---
        draw_blood_stains(&world.blood_stains, world.game_time);
        draw_portals(&world.portals, world.game_time);

        // --- 对象层 ---
        draw_food(world.food);
        draw_fruits(&world.fruits, &world.registry, world.game_time);
        draw_dropped_foods(&world.ai_manager.dropped_foods, world.game_time);

        // --- 计算插值 ---
        let blend = if world.state == GameState::Playing && !world.buff_state.frozen {
            (accumulator / tick).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // --- 特效层 ---
        draw_afterimages(&world.afterimages, world.game_time);

        if world.buff_state.shield_active {
            draw_shield_effect(&world.snake.body, world.game_time);
        }
        if world.buff_state.ghost_active {
            draw_ghost_effect(&world.snake.body, world.game_time);
        }

        // --- 蛇 ---
        if world.buff_state.sandworm_phase != SandwormPhase::None {
            draw_sandworm_mode(&world.snake.body, &world.buff_state, world.game_time);
        } else {
            draw_snake(
                &world.snake,
                &world.buff_state,
                world.damage_state.phase,
                world.game_time,
                world.wrap,
                blend,
            );
        }
        
        // --- AI 蛇 ---
        draw_ai_snakes(&world.ai_manager.snakes, world.game_time, blend);

        // --- 粒子层 ---
        draw_particles(&world.particles);

        // --- 覆盖层 ---
        if world.state == GameState::GameOver {
            if world.score > world.high_score {
                world.high_score = world.score;
            }
            if !over_once {
                sounds.play_game_over();
                over_once = true;
            }
            draw_overlay("Game Over", "Enter/R to restart, Esc to quit");
        }
        if world.state == GameState::Paused {
            draw_overlay("Paused", "Space to resume, Enter/R to restart");
        }

        // --- HUD层 ---
        draw_hud(
            world.score,
            world.high_score,
            &world.combo_state,
            &world.buff_state,
        );

        // --- 等待下一帧 ---
        next_frame().await;
    }
}
