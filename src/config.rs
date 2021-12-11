use std::fs::File;

use ron::de::from_reader;

use once_cell::sync::Lazy;
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
