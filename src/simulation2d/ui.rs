use bevy::prelude::*;

use super::resources::{EvaluationStopwatch, FitnessStats, GenerationCount};

struct TimerText;
struct GenerationText;
struct FitnessText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_timer_text.system())
            .add_system(update_generation_text.system())
            .add_system(update_fitness_stats_text.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let transparent = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: Rect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: transparent.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    material: transparent.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Generation: 0",
                                TextStyle {
                                    font: font.clone(),
                                    color: Color::WHITE,
                                    font_size: 60.0,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(GenerationText);
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Time: 0.0",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(TimerText);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    material: transparent.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                ..Default::default()
                            },
                            text: Text {
                                alignment: TextAlignment {
                                    horizontal: HorizontalAlign::Right,
                                    ..Default::default()
                                },
                                sections: vec![
                                    TextSection {
                                        value: "Best: {}\n".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 60.0,
                                            color: Color::GREEN,
                                        },
                                    },
                                    TextSection {
                                        value: "Worst: {}\n".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 60.0,
                                            color: Color::RED,
                                        },
                                    },
                                    TextSection {
                                        value: "Average {}\n".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                        },
                                    },
                                ],
                            },
                            ..Default::default()
                        })
                        .insert(FitnessText);
                });
        });
}

fn update_generation_text(
    generation_count: Res<GenerationCount>,
    mut query: Query<&mut Text, With<GenerationText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Generation: {}", generation_count.0);
    }
}

fn update_timer_text(
    stopwatch: Res<EvaluationStopwatch>,
    mut query: Query<&mut Text, With<TimerText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", stopwatch.0.elapsed_secs());
    }
}

fn update_fitness_stats_text(
    fitness_stats: Res<FitnessStats>,
    mut query: Query<&mut Text, With<FitnessText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Best: {:.2}\n", fitness_stats.best);
        text.sections[1].value = format!("Worst: {:.2}\n", fitness_stats.worst);
        text.sections[2].value = format!("Average: {:.2}\n", fitness_stats.average);
    }
}
