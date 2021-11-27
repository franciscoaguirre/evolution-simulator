use bevy::prelude::*;

pub struct Muscle {
    pub min_length: f32,
    pub max_length: f32,
    pub strength: f32,
    pub nodes: (usize, usize),
}

pub fn create_muscle(parent: &mut ChildBuilder, muscle: &Muscle) {
    println!("I'm here!");
}
