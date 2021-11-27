/// Represents the characteristics of a Muscle
pub struct MusclePhenotype {
    /// Time the muscle is extending
    pub extended_time: f32,
    /// Time the muscle is contracting
    pub contracted_time: f32,
    /// Maximum length the muscle can reach
    pub extended_length: f32,
    /// Minimum length
    pub contracted_length: f32,
    /// The strength with which it pulls/pushes its nodes together/apart
    pub strength: f32,
    /// The nodes this muscle pulls/pushes
    pub nodes: (usize, usize),
}
