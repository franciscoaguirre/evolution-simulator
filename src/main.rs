use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  // plane
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..Default::default()
  });
  // cube
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..Default::default()
  });
  // light
  commands.spawn_bundle(LightBundle {
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  });
  // camera
  commands
    .spawn_bundle(PerspectiveCameraBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCamera::default());
}

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(FlyCameraPlugin)
    .add_startup_system(setup.system())
    .run();
}
