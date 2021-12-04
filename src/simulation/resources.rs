use bevy::core::Stopwatch;

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
