use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::genetic_algorithm::node_phenotype::NodePhenotype;

pub struct Node {
    pub friction: f32,
}

pub fn create_node(
    parent: &mut ChildBuilder,
    node_phenotype: &NodePhenotype,
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
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .id()
}
