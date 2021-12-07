use bevy::prelude::{shape as bevyShape, *};
use bevy_rapier3d::prelude::*;

use crate::genetic_algorithm::node_phenotype::NodePhenotype;

pub struct Node {
    pub friction: f32,
}

pub fn create_node(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_phenotype: &NodePhenotype,
    position_offset: Vec3,
    node_size: f32,
) -> Entity {
    let rigid_body = RigidBodyBundle {
        position: (node_phenotype.position + position_offset).into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        position: (node_phenotype.position + position_offset).into(),
        shape: ColliderShape::ball(node_size / 2.0),
        material: ColliderMaterial {
            friction: 800.0,
            ..Default::default()
        },
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(0b10, 0b01),
            ..Default::default()
        },
        ..Default::default()
    };

    parent
        .spawn()
        .insert(Node {
            friction: node_phenotype.friction,
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevyShape::Icosphere {
                radius: node_size / 2.0,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .id()
}
