//! 游戏状态枚举

/// 游戏状态枚举
///
/// 控制游戏的主循环行为：
/// - Playing: 正常游戏进行中，响应输入和更新逻辑
/// - Paused: 暂停状态，显示暂停覆盖层
/// - GameOver: 游戏结束，显示结束画面
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}
