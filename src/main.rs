use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;

mod prefabs;
use prefabs::{creature, muscle::MusclePlugin, plane};

mod genetic_algorithm;
use genetic_algorithm::{
    creature_chromosome::CreatureChromosome, muscle_phenotype::MusclePhenotype,
    node_phenotype::NodePhenotype,
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
        // NodePhenotype {
        //     position: Vec3::new(2.5, 5.0, 2.5),
        //     friction: 1.5,
        // },
    ];
    let muscles = vec![
        MusclePhenotype {
            extended_time: 1.0,
            contracted_time: 1.0,
            contracted_length: 1.0,
            extended_length: 8.0,
            strength: 1.0,
            nodes: (0, 1),
        },
        // MusclePhenotype {
        //     extended_time: 1.5,
        //     contracted_time: 0.5,
        //     contracted_length: 3.0,
        //     extended_length: 12.0,
        //     strength: 15.0,
        //     nodes: (0, 2),
        // },
        // MusclePhenotype {
        //     extended_time: 1.3,
        //     contracted_time: 0.7,
        //     extended_length: 6.0,
        //     strength: 10.0,
        //     contracted_length: 1.0,
        //     nodes: (1, 2),
        // },
    ];
    let creature = CreatureChromosome {
        nodes,
        muscles,
        ..Default::default()
    };
    creature::create_creature(
        &mut commands,
        &mut meshes,
        &mut materials,
        creature,
        Vec3::ZERO,
    );
    plane::create_plane(&mut commands, &mut meshes, &mut materials);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(MusclePlugin)
        .add_startup_system(setup.system())
        .run();
}
