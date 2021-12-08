use bevy::prelude::*;

pub fn create_plane(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("ground.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        sprite: Sprite {
            size: Vec2::new(10.0, 0.1),
            resize_mode: SpriteResizeMode::Manual,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
        ..Default::default()
    });
}
