#![feature(once_cell)]
use std::time::Duration;

use arguments::Opt;
use bevy::{
    app::ScheduleRunnerSettings, diagnostic::FrameTimeDiagnosticsPlugin, input, prelude::*,
    render::camera::OrthographicProjection,
};
mod arguments;
mod config;
mod simulation2d;

use simulation2d::{plane::create_plane, plugin::SimulationPlugin, ui::UIPlugin};

mod genetic_algorithm;
use genetic_algorithm::plugin::GeneticAlgorithmPlugin;
use structopt::StructOpt;

struct CameraTransform {
    relative_zoom: f32,
    position: Vec2,
}

struct MainCamera;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    create_plane(&mut commands, &mut materials, asset_server);
}

fn camera_movement(keys: Res<Input<KeyCode>>, mut camera: ResMut<CameraTransform>) {
    const MIN_ZOOM: f32 = 0.001;
    const MAX_ZOOM: f32 = 1.0;
    const MOUSE_ZOOM_RATIO: f32 = 0.005;
    const MOVE_RATIO: f32 = 0.1;

    let delta_zoom = if keys.pressed(KeyCode::M) {
        1.0
    } else if keys.pressed(KeyCode::N) {
        -1.0
    } else {
        0.0
    };

    let horizontal_movement = if keys.pressed(KeyCode::D) {
        0.1
    } else if keys.pressed(KeyCode::A) {
        -0.1
    } else {
        0.0
    };

    let vertical_movement = if keys.pressed(KeyCode::W) {
        0.1
    } else if keys.pressed(KeyCode::S) {
        -0.1
    } else {
        0.0
    };

    camera.relative_zoom =
        (camera.relative_zoom + delta_zoom * MOUSE_ZOOM_RATIO).clamp(MIN_ZOOM, MAX_ZOOM);

    camera.position += Vec2::new(horizontal_movement, vertical_movement) * MOVE_RATIO;
}

fn move_camera(
    camera_transform: Res<CameraTransform>,
    mut cameras: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    const DEFAULT_ZOOM: f32 = 0.01;

    if !camera_transform.is_changed() {
        return;
    }

    for (mut camera, mut transform) in cameras.iter_mut() {
        camera.scale = DEFAULT_ZOOM + camera_transform.relative_zoom;

        transform.translation = Vec3::new(
            camera_transform.position.x,
            camera_transform.position.y,
            0.0,
        );
    }
}

fn main() {
    let options = Opt::from_args();

    let mut app = App::build();

    if options.headless {
        println!("Running in headless mode");

        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins);
    } else {
        app.add_plugins(DefaultPlugins)
            .insert_resource(CameraTransform {
                relative_zoom: 0.0,
                position: Vec2::new(0.0, 0.0),
            })
            .add_system(camera_movement.system())
            .add_system(move_camera.system())
            .add_system(input::system::exit_on_esc_system.system())
            .add_startup_system(setup.system())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(UIPlugin);
    }

    app.add_plugin(SimulationPlugin)
        .add_plugin(GeneticAlgorithmPlugin);

    app.run();
}
