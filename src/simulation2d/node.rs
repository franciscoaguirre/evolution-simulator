use bevy::prelude::*;

use crate::genetic_algorithm::node_phenotype::NodePhenotype;

use super::physics::Velocity;

pub struct Node {
    pub radius: f32,
    pub friction: f32,
}

#[derive(Bundle)]
pub struct NodeBundle {
    node: Node,
    velocity: Velocity,

    #[bundle]
    sprite: SpriteBundle,
}

pub fn create_node(
    parent: &mut ChildBuilder,
    node_phenotype: &NodePhenotype,
    _meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    node_size: f32,
) -> Entity {
    let texture_handle = asset_server.load("circle.png");

    parent
        .spawn()
        .insert(Node {
            radius: node_size,
            friction: node_phenotype.friction,
        })
        .insert(Velocity(Vec3::default()))
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            sprite: Sprite {
                size: Vec2::new(node_size / 2.0, node_size / 2.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                node_phenotype.position.x,
                node_phenotype.position.y,
                0.0,
            )),
            ..Default::default()
        })
        .id()
}

pub fn create_node_bundle(
    parent: &mut ChildBuilder,
    node_phenotype: &NodePhenotype,
    _meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    node_size: f32,
) -> NodeBundle {
    let texture_handle = asset_server.load("circle.png");

    NodeBundle {
        node: Node {
            radius: node_size,
            friction: node_phenotype.friction,
        },
        velocity: Velocity(Vec3::default()),
        sprite: SpriteBundle {
            material: materials.add(texture_handle.into()),
            sprite: Sprite {
                size: Vec2::new(node_size / 2.0, node_size / 2.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                node_phenotype.position.x,
                node_phenotype.position.y,
                0.0,
            )),
            ..Default::default()
        },
    }
}
