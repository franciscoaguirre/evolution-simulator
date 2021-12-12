use std::fs::File;

use bevy::{app::AppExit, prelude::*};
use ron::de::from_reader;

use crate::{
    config::CONFIG,
    genetic_algorithm::{
        creature_chromosome::CreatureChromosome,
        operations::{Correctable, RandomCreatable},
    },
};

use super::{creature::create_creature, resources::EvaluationStopwatch};

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_creature_from_file.system())
            .add_system(check_should_end.system());
    }
}

fn load_creature_from_file(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let chromosome = match read_chromosome() {
        Ok(mut chromosome) => {
            chromosome.correct();
            chromosome
        }
        Err(error) => {
            warn!(
                "Chromosome file error. {}. Using random chromosome.",
                error.code
            );
            CreatureChromosome::random()
        }
    };
    let node_size = CONFIG.node_size;
    create_creature(
        &mut commands,
        chromosome,
        &mut meshes,
        &mut materials,
        &asset_server,
        node_size,
    );
}

fn read_chromosome() -> Result<CreatureChromosome, ron::error::Error> {
    let input_path = "chromosome.ron";
    let file = File::open(&input_path)?;
    let chromosome: CreatureChromosome = from_reader(file)?;
    Ok(chromosome)
}

fn check_should_end(
    evaluation_stopwatch: Res<EvaluationStopwatch>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if evaluation_stopwatch.0.elapsed_secs() >= CONFIG.evaluation_time {
        app_exit_events.send(AppExit);
    }
}
