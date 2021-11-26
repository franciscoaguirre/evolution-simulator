use bevy::prelude::{shape as bevyShape, *};
use bevy_rapier3d::prelude::*;

pub struct Node {
    pub position: Vec3,
    pub mass: f32,
}

pub fn create_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn()
        .insert(Node {
            position: Vec3::new(0.0, 10.0, 0.0),
            mass: 1.0,
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .with_children(|parent| {
            // child sphere
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(bevyShape::Icosphere {
                    radius: 0.5,
                    subdivisions: 4,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    ..Default::default()
                }),
                ..Default::default()
            });
        })
        .insert(ColliderPositionSync::Discrete);
}
