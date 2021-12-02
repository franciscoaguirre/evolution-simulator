use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;

mod simulation;
use simulation::{plane::create_plane, plugin::SimulationPlugin, logger::LoggerPlugin};

mod genetic_algorithm;
use genetic_algorithm::plugin::GeneticAlgorithmPlugin;

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
        .add_plugin(LoggerPlugin)
        .add_startup_system(setup.system())
        .run();
}
