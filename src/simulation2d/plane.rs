use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn create_plane(
    commands: &mut Commands,
) {
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1),
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(0b01, 0b11),
            ..Default::default()
        },
        material: ColliderMaterial {
            friction: 800.0,
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(2));
}
