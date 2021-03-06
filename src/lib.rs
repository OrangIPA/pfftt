mod player;
mod ground;

use bevy::prelude::*;

use player::*;
use ground::*;

pub const SCALE: f32 = 2.;

pub struct PffttPlugins;

impl Plugin for PffttPlugins {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_ground)
            .add_system(player_input)
            .add_system(player_update)
            .add_system(animate_player)
            ;
    }
}

pub fn setup() {}
