use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_rapier3d::prelude::*;

mod prefabs;
use prefabs::{creature, node, muscle, plane};

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
        node::Node { position: Vec3::new(0.0, 0.5, 0.0), mass: 1.0 },
        node::Node { position: Vec3::new(5.0, 0.5, 5.0), mass: 1.0 },
    ];
    let muscles = vec![
        muscle::Muscle { min_length: 1.0, max_length: 1.0, strength: 1.0, nodes: (0, 1) }
    ];
    let creature = creature::Creature { nodes, muscles, ..Default::default() };
    creature::create_creature(&mut commands, &mut meshes, &mut materials, creature);
    plane::create_plane(&mut commands, &mut meshes, &mut materials);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .run();
}
