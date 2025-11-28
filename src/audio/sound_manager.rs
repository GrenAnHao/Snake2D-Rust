//! # 音效管理器
//!
//! 提供游戏音效的加载和播放功能。
//!
//! ## 实现原理
//!
//! 音效使用 `make_tone_wav` 函数在运行时生成：
//!
//! 1. 计算正弦波采样值
//! 2. 构建 WAV 文件头
//! 3. 写入 PCM 数据
//! 4. 加载到 Macroquad 音频系统
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! let sounds = SoundManager::new().await;
//! sounds.play_eat();   // 吃食物
//! sounds.play_trap();  // 吃陷阱
//! sounds.play_power(); // 吃功能果实
//! ```

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

/// 音效管理器
///
/// 管理所有游戏音效的加载和播放。
///
/// ## 音效说明
///
/// - `eat_sound`: 高频短促音，表示正面反馈
/// - `trap_sound`: 低频较长音，表示负面效果
/// - `power_sound`: 中频音，表示获得能力
/// - `over_sound`: 最低频长音，表示游戏结束
pub struct SoundManager {
    /// 吃食物音效 (880Hz, 0.08s)
    pub eat_sound: Sound,
    /// 陷阱音效 (330Hz, 0.15s)
    pub trap_sound: Sound,
    /// 功能果实音效 (660Hz, 0.2s)
    pub power_sound: Sound,
    /// 游戏结束音效 (220Hz, 0.3s)
    pub over_sound: Sound,
}

impl SoundManager {
    /// 创建并加载所有音效
    pub async fn new() -> Self {
        let eat_sound = load_sound_from_bytes(&make_tone_wav(880.0, 0.08, 0.4)).await.unwrap();
        let trap_sound = load_sound_from_bytes(&make_tone_wav(330.0, 0.15, 0.5)).await.unwrap();
        let power_sound = load_sound_from_bytes(&make_tone_wav(660.0, 0.2, 0.6)).await.unwrap();
        let over_sound = load_sound_from_bytes(&make_tone_wav(220.0, 0.30, 0.5)).await.unwrap();

        SoundManager {
            eat_sound,
            trap_sound,
            power_sound,
            over_sound,
        }
    }

    /// 播放吃食物音效
    pub fn play_eat(&self) {
        play_sound(&self.eat_sound, PlaySoundParams { looped: false, volume: 0.5 });
    }

    /// 播放陷阱音效
    pub fn play_trap(&self) {
        play_sound(&self.trap_sound, PlaySoundParams { looped: false, volume: 0.6 });
    }

    /// 播放功能果实音效
    pub fn play_power(&self) {
        play_sound(&self.power_sound, PlaySoundParams { looped: false, volume: 0.7 });
    }

    /// 播放游戏结束音效
    pub fn play_game_over(&self) {
        play_sound(&self.over_sound, PlaySoundParams { looped: false, volume: 0.7 });
    }
}

/// 生成简单的单声道 16-bit PCM WAV 音频数据（正弦波）
///
/// ## 算法说明
///
/// 使用正弦波公式生成音频采样：
/// ```text
/// sample[n] = sin(2π × freq × n / sample_rate) × volume × MAX_INT16
/// ```
///
/// ## WAV 文件结构
///
/// ```text
/// ┌────────────────────────────────────┐
/// │ RIFF Header (12 bytes)             │
/// │ - "RIFF" + file_size + "WAVE"      │
/// ├────────────────────────────────────┤
/// │ fmt Chunk (24 bytes)               │
/// │ - 格式信息: PCM, 单声道, 44100Hz   │
/// ├────────────────────────────────────┤
/// │ data Chunk (8 + data_size bytes)   │
/// │ - "data" + size + PCM samples      │
/// └────────────────────────────────────┘
/// ```
///
/// # 参数
/// - `freq`: 频率 (Hz)，人耳可听范围 20-20000Hz
/// - `dur`: 持续时间 (秒)
/// - `vol`: 音量 (0.0-1.0)，会被 clamp 到有效范围
///
/// # 返回
/// 完整的 WAV 格式字节数组，可直接加载播放
///
/// # 示例
///
/// ```rust,ignore
/// // 生成 A4 音符 (440Hz)，持续 0.5 秒，50% 音量
/// let wav_data = make_tone_wav(440.0, 0.5, 0.5);
/// ```
pub fn make_tone_wav(freq: f32, dur: f32, vol: f32) -> Vec<u8> {
    let sr = 44100;
    let samples = (dur * sr as f32) as usize;
    let mut pcm: Vec<i16> = Vec::with_capacity(samples);

    for n in 0..samples {
        let t = n as f32 / sr as f32;
        let s = (2.0 * std::f32::consts::PI * freq * t).sin();
        let v = (s * vol.clamp(0.0, 1.0) * std::i16::MAX as f32) as i16;
        pcm.push(v);
    }

    let data_size = (pcm.len() * 2) as u32;
    let mut out: Vec<u8> = Vec::with_capacity(44 + data_size as usize);

    // RIFF header
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_size).to_le_bytes());
    out.extend_from_slice(b"WAVE");

    // fmt chunk
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&(sr as u32).to_le_bytes());
    let byte_rate = sr as u32 * 2;
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    for s in pcm {
        out.extend_from_slice(&s.to_le_bytes());
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_tone_wav_header() {
        let wav = make_tone_wav(440.0, 0.1, 0.5);

        // 检查 RIFF header
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");

        // 检查 fmt chunk
        assert_eq!(&wav[12..16], b"fmt ");

        // 检查 data chunk
        assert_eq!(&wav[36..40], b"data");
    }

    #[test]
    fn test_make_tone_wav_size() {
        let dur = 0.1;
        let wav = make_tone_wav(440.0, dur, 0.5);

        // 预期样本数
        let expected_samples = (dur * 44100.0) as usize;
        let expected_data_size = expected_samples * 2;
        let expected_total = 44 + expected_data_size;

        assert_eq!(wav.len(), expected_total);
    }
}


#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // **Feature: modular-migration, Property 5: WAV Generation Format Validity**
    // *For any* valid frequency (20-20000 Hz), duration (0.01-10s), and volume (0-1),
    // the generated WAV data should have correct header format.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_wav_format_validity(
            freq in 20.0f32..20000.0,
            dur in 0.01f32..1.0,  // 限制在1秒内避免测试太慢
            vol in 0.0f32..1.0,
        ) {
            let wav = make_tone_wav(freq, dur, vol);

            // 验证 RIFF header
            prop_assert_eq!(&wav[0..4], b"RIFF", "Missing RIFF header");
            prop_assert_eq!(&wav[8..12], b"WAVE", "Missing WAVE format");

            // 验证 fmt chunk
            prop_assert_eq!(&wav[12..16], b"fmt ", "Missing fmt chunk");

            // 验证 data chunk
            prop_assert_eq!(&wav[36..40], b"data", "Missing data chunk");

            // 验证文件大小
            let expected_samples = (dur * 44100.0) as usize;
            let expected_data_size = expected_samples * 2;
            let expected_total = 44 + expected_data_size;
            prop_assert_eq!(wav.len(), expected_total, "Incorrect WAV size");

            // 验证 RIFF chunk size
            let riff_size = u32::from_le_bytes([wav[4], wav[5], wav[6], wav[7]]);
            prop_assert_eq!(riff_size as usize, wav.len() - 8, "Incorrect RIFF chunk size");

            // 验证 data chunk size
            let data_size = u32::from_le_bytes([wav[40], wav[41], wav[42], wav[43]]);
            prop_assert_eq!(data_size as usize, expected_data_size, "Incorrect data chunk size");
        }
    }
}
