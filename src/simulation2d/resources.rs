use bevy::core::Stopwatch;

#[derive(Default)]
pub struct EvaluationStopwatch(pub Stopwatch);

#[derive(Default)]
pub struct RealTimeStopwatch(pub Stopwatch);

#[derive(Default)]
pub struct GenerationCount(pub usize);
