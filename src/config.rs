use std::fs::File;

use ron::de::from_reader;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    // World settings
    pub evaluation_time: f32,
    pub time_scale: f32,
    pub fixed_time_step: f32,
    pub gravity: f32,
    pub air_friction: f32,
    pub node_size: f32,

    // Min and max values settings
    pub min_friction: f32,
    pub max_friction: f32,
    pub min_strength: f32,
    pub max_strength: f32,
    pub min_contracted_length: f32,
    pub max_contracted_length: f32,
    pub min_extended_length: f32,
    pub max_extended_length: f32,
    pub min_contracted_time: f32,
    pub max_contracted_time: f32,

    // Genetic algorithm settings
    pub max_unchanged_generations: usize,
    pub improvement_threshold: f32,

    // Mutation settings
    pub elimination_mutation_chance_modifier: f32,
    pub position_mutation_chance_modifier: f32,
    pub single_value_mutation_chance_modifier: f32,
    pub creation_mutation_chance_modifier: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            time_scale: 10.0,
            fixed_time_step: 0.01,
            evaluation_time: 15.0,
            gravity: 10.0,
            air_friction: 8.0,
            node_size: 0.08,

            min_friction: 0.3,
            max_friction: 1.0,
            min_strength: 0.5,
            max_strength: 10.0,
            min_contracted_length: 0.08,
            max_contracted_length: 0.40,
            min_extended_length: 0.08,
            max_extended_length: 0.40,
            min_contracted_time: 0.1,
            max_contracted_time: 1.0,

            max_unchanged_generations: 10,
            improvement_threshold: 0.05,

            elimination_mutation_chance_modifier: 0.5,
            position_mutation_chance_modifier: 1.0,
            single_value_mutation_chance_modifier: 1.0,
            creation_mutation_chance_modifier: 3.0,
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| load_config());

fn load_config() -> Config {
    match load_config_from_file() {
        Ok(x) => {
            println!("Loaded config from file");
            x
        }
        Err(err) => {
            println!(
                "Config file error. {}. Using default config.",
                err.code.to_string()
            );
            Config::default()
        }
    }
}

fn load_config_from_file() -> Result<Config, ron::error::Error> {
    let input_path = "config.ron";
    let file = File::open(&input_path)?;
    let config: Config = from_reader(file)?;
    Ok(config)
}
