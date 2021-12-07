use bevy::prelude::{*, shape as bevyShape};
use bevy_rapier2d::prelude::*;

use crate::genetic_algorithm::node_phenotype::NodePhenotype;

pub struct Node {
    pub friction: f32,
}

pub fn create_node(
    parent: &mut ChildBuilder,
    node_phenotype: &NodePhenotype,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node_size: f32,
) -> Entity {
    let rigid_body = RigidBodyBundle {
        position: node_phenotype.position.into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let collider = ColliderBundle {
        position: node_phenotype.position.into(),
        shape: ColliderShape::ball(node_size),
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
        .insert(ColliderDebugRender {
            color: Color::Rgba { red: 255.0, green: 0.0, blue: 0.0, alpha: 1.0 }
        })
        .insert(ColliderPositionSync::Discrete)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevyShape::Icosphere {
                radius: node_size,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..Default::default()
            }),
            ..Default::default()
        })
        .id()
}
