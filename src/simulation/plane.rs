use bevy::prelude::{shape as bevyShape, *};
use bevy_rapier3d::prelude::*;

pub fn create_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1, 100.0),
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(0b01, 0b11),
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevyShape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            ..Default::default()
        });
}
