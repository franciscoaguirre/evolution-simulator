use bevy::core::Stopwatch;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub evaluation_time: f32,
    pub time_scale: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            evaluation_time: 15.0,
            time_scale: 1.0,
        }
    }
}

#[derive(Default)]
pub struct EvaluationStopwatch(pub Stopwatch);

#[derive(Default)]
pub struct GenerationCount(pub usize);

#[derive(Default)]
pub struct FitnessStats {
    pub best: f32,
    pub worst: f32,
    pub average: f32,
}
