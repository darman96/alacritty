use alacritty_config_derive::ConfigDeserialize;
use serde::Serialize;

use crate::config::ui_config::Percentage;

/// Configuration for retro terminal effects (scanlines + glow).
#[derive(ConfigDeserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RetroEffectConfig {
    /// Whether the retro effect is enabled.
    pub enabled: bool,

    /// Intensity of the scanline effect (0.0 = no scanlines, 1.0 = full black lines).
    pub scanline_intensity: Percentage,

    /// Intensity of the glow/bloom around text (0.0 = no glow, 1.0 = maximum glow).
    pub glow_intensity: Percentage,

    /// Thickness of each scanline in pixels.
    pub scanline_thickness: f32,

    /// Distance between scanlines in pixels (the bright gap).
    pub scanline_spacing: f32,
}

impl Default for RetroEffectConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            scanline_intensity: Percentage::new(0.5),
            glow_intensity: Percentage::new(0.3),
            scanline_thickness: 1.0,
            scanline_spacing: 2.0,
        }
    }
}
