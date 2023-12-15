use ggez::conf::NumSamples;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::CustomError;

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub game_title: String,
    pub cell_size: f32,
    pub win_height: f32,
    pub win_width: f32,
    pub grid_width: usize,
    pub grid_height: usize,
    pub base_enemy_speed: f32,
    pub enemy_collision_radius: f32,
    pub enemy_scaling_factor: f32,
    pub vsync: bool,
    #[serde(deserialize_with = "deserialize_num_samples")]
    pub multi_sampling: NumSamples,
}

impl GameConfig {
    pub fn load_from_file(file_name: &str) -> Result<GameConfig, CustomError> {
        println!("loading configuration from {}", file_name);
        let config_str = read_to_string("game_config.json").map_err(CustomError::IoError)?;

        let game_config: GameConfig =
            serde_json::from_str(&config_str).map_err(CustomError::LoadGameDataError)?;
        Ok(game_config)
    }
}

fn deserialize_num_samples<'de, D>(deserializer: D) -> Result<NumSamples, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let samples = u8::deserialize(deserializer)?;
    Ok(u8_to_num_samples(samples))
}

fn u8_to_num_samples(samples: u8) -> NumSamples {
    match samples {
        1=>NumSamples::One,
        4=>NumSamples::Four,
        _=>panic!("Invalid multi_sampling value. Must be 1 or 4")
    }
}
