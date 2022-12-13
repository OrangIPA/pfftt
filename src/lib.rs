mod player;
mod ground;
mod camera;

use bevy::prelude::*;

use player::*;
use ground::*;
use camera::follow_player;

pub const SCALE: f32 = 3.;

pub struct PffttPlugins;

impl Plugin for PffttPlugins {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_ground)
            .add_system(player_input)
            .add_system(player_update)
            .add_system(animate_player)
            .add_system(follow_player)
            .add_system(fall_to_the_void)
            ;
    }
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(camera::Camera);
}
