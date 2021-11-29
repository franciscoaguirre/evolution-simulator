use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;

mod simulation;
use simulation::{
    creature::create_creature, plane::create_plane, plugin::SimulationPlugin,
};

mod genetic_algorithm;
use genetic_algorithm::{
    creature_chromosome::CreatureChromosome, muscle_phenotype::MusclePhenotype,
    node_phenotype::NodePhenotype, plugin::GeneticAlgorithmPlugin,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..Default::default()
    });
    let nodes = vec![
        NodePhenotype {
            position: Vec3::new(0.0, 0.5, 0.0),
            friction: 1.0,
        },
        NodePhenotype {
            position: Vec3::new(5.0, 0.5, 5.0),
            friction: 2.0,
        },
    ];
    let muscles = vec![MusclePhenotype {
        contracted_time: 5.0,
        contracted_length: 13.0,
        extended_length: 15.0,
        strength: 10.0,
        nodes: (0, 1),
    }];
    let creature = CreatureChromosome {
        nodes,
        muscles,
        ..Default::default()
    };
    create_creature(
        &mut commands,
        &mut meshes,
        &mut materials,
        creature,
        Vec3::default(),
    );
    create_plane(&mut commands, &mut meshes, &mut materials);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(GeneticAlgorithmPlugin)
        .add_startup_system(setup.system())
        .run();
}
