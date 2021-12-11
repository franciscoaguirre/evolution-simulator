use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::resources::{EvaluationStopwatch, GenerationCount};

struct TimerText;
struct GenerationText;
struct FPSText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_timer_text.system())
            .add_system(update_generation_text.system())
            .add_system(update_frames_stats.system());
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
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                material: transparent.clone(),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                        flex_direction: FlexDirection::ColumnReverse,
                        // justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    material: transparent.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "FPS: {}\n".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 40.0,
                                            color: Color::WHITE,
                                        },
                                    },
                                    TextSection {
                                        value: "FRAME TIME: {}\n".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 40.0,
                                            color: Color::WHITE,
                                        },
                                    },
                                ],
                                alignment: TextAlignment {
                                    horizontal: HorizontalAlign::Left,
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        })
                        .insert(FPSText);
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

fn update_frames_stats(
    diagnostics: Res<Diagnostics>,
    mut fps_text: Query<&mut Text, With<FPSText>>,
) {
    for mut text in fps_text.iter_mut() {
        if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = diagnostic.value() {
                text.sections[0].value = format!("AVG FPS: {:.4}\n", value);
            }
        }

        if let Some(diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(value) = diagnostic.value() {
                text.sections[1].value = format!("FRAME TIME: {:.4}\n", value);
            }
        }
    }
}
