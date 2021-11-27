use bevy::prelude::{shape as bevyShape, *};
use bevy_rapier3d::prelude::*;

pub struct Node {
    pub position: Vec3,
    pub mass: f32,
}

pub fn create_node(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node: &Node,
    position_offset: Vec3,
) {
    let rigid_body = RigidBodyBundle {
        ..Default::default()
    };
    let collider = ColliderBundle {
        position: (node.position + position_offset).into(),
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };

    parent
        .spawn()
        // .insert(node)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevyShape::Icosphere {
                radius: 0.5,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
