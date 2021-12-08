use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::OrthographicProjection,
};
use bevy_fly_camera::FlyCameraPlugin;
use bevy_rapier2d::prelude::*;

mod simulation2d;
use simulation2d::{plane::create_plane, plugin::SimulationPlugin};

mod genetic_algorithm;
use genetic_algorithm::plugin::GeneticAlgorithmPlugin;

struct CameraTransform {
    relative_zoom: f32,
    position: Vec2,
}

struct MainCamera;

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    // .insert(FlyCamera2d::default());

    create_plane(&mut commands);
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
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(SimulationPlugin)
        .add_plugin(GeneticAlgorithmPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .insert_resource(CameraTransform {
            relative_zoom: 0.0,
            position: Vec2::new(0.0, 0.0),
        })
        .add_system(camera_movement.system())
        .add_system(move_camera.system())
        .run();
}
