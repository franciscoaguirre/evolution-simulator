use bevy::prelude::*;
use bevy_rapier3d::prelude::ColliderPosition;

use super::{creature::Creature, node, plugin::calculate_creatures_position};

pub struct LoggerPlugin;

struct FitnessText;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_node_fitness_text.system())
            .add_system(log_nodes_fitness.system());
    }
}

fn setup_node_fitness_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Best: {} \n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "Worst: {} \n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "Average {} \n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FitnessText);
}

fn log_nodes_fitness(
    creatures: Query<(Entity, &Creature)>,
    collider_node_positions: Query<(&ColliderPosition, &Parent), With<node::Node>>,
    mut query: Query<&mut Text, With<FitnessText>>,
) {
    let mut fitnesses: Vec<f32> = Vec::new();

    for (entity, creature) in creatures.iter() {
        let position = calculate_creatures_position(entity, &collider_node_positions);

        let fitness = (creature.starting_position - position).length();

        if fitness.is_nan() || fitness.is_infinite() {
            continue;
        }

        fitnesses.push((creature.starting_position - position).length());
    }

    let best = fitnesses
        .iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap_or(&0.0);
    let worst = fitnesses
        .iter()
        .min_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap_or(&0.0);
    let average = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Best: {:.2}", best);
        text.sections[1].value = format!("Worst: {:.2}", worst);
        text.sections[2].value = format!("Average: {:.2}", average);
    }
}
