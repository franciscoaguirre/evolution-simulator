use bevy::core::Stopwatch;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub evaluation_time: f32,
    pub time_scale: f32,
    pub gravity: f32,
    pub population_size: usize,
    pub air_friction: f32,
    pub node_size: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            evaluation_time: 15.0,
            time_scale: 1.0,
            gravity: 0.0,
            population_size: 4,
            air_friction: 1.05,
            node_size: 0.1,
        }
    }
}

#[derive(Default)]
pub struct EvaluationStopwatch(pub Stopwatch);

#[derive(Default)]
pub struct RealTimeStopwatch(pub Stopwatch);

#[derive(Default)]
pub struct GenerationCount(pub usize);
