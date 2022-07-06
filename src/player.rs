use bevy::prelude::*;

use crate::SCALE;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(f32);

#[derive(Component, Deref, DerefMut)]
pub struct Acceleration(f32);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0., -24. * SCALE, 0.),
            scale: Vec3::new(SCALE, SCALE, 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity(0.))
    .insert(Acceleration(0.));
}
pub fn player_movement(
    mut query: Query<(&mut Velocity, &mut Acceleration), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::D) {

    }
}