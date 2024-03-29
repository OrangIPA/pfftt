use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_player);
    }
}

#[derive(Component)]
pub struct Camera;

pub fn follow_player(
    mut cam: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut cam = cam.single_mut();
    let player = player.single();
    cam.translation.x += (player.translation.x - cam.translation.x) * 0.2;
    cam.translation.y += (player.translation.y - cam.translation.y) * 0.2;
}