use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::touch::TriggerCorner;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    // Direct mapping to CLI args - no arbitrary grouping
    pub engine: Option<String>,
    pub engine_base_url: Option<String>,
    pub engine_api_key: Option<String>,
    pub model: String,
    pub prompt: String,
    pub no_submit: bool,
    pub no_draw: bool,
    pub no_svg: bool,
    pub no_keyboard: bool,
    pub no_draw_progress: bool,
    pub input_png: Option<String>,
    pub output_file: Option<String>,
    pub model_output_file: Option<String>,
    pub save_screenshot: Option<String>,
    pub save_bitmap: Option<String>,
    pub no_loop: bool,
    pub no_trigger: bool,
    pub apply_segmentation: bool,
    pub web_search: bool,
    pub thinking: bool,
    pub thinking_tokens: u32,
    pub log_level: String,
    pub trigger_corner: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            engine: None,
            engine_base_url: None,
            engine_api_key: None,
            model: "claude-sonnet-4-0".to_string(),
            prompt: "general.json".to_string(),
            no_submit: false,
            no_draw: false,
            no_svg: false,
            no_keyboard: false,
            no_draw_progress: false,
            input_png: None,
            output_file: None,
            model_output_file: None,
            save_screenshot: None,
            save_bitmap: None,
            no_loop: false,
            no_trigger: false,
            apply_segmentation: false,
            web_search: false,
            thinking: false,
            thinking_tokens: 5000,
            log_level: "info".to_string(),
            trigger_corner: "UR".to_string(),
        }
    }
}

impl Config {
    /// Validate the configuration and return any errors
    pub fn validate(&self) -> Result<()> {
        // Validate trigger corner
        TriggerCorner::from_string(&self.trigger_corner)?;

        // Validate log level
        match self.log_level.as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => {},
            _ => return Err(anyhow::anyhow!("Invalid log level: {}", self.log_level)),
        }

        // Validate thinking tokens
        if self.thinking_tokens == 0 {
            return Err(anyhow::anyhow!("thinking_tokens must be greater than 0"));
        }

        Ok(())
    }

}