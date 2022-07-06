mod player;
mod ground;

use bevy::prelude::*;

use player::*;
use ground::*;

pub const SCALE: i32 = 3;

pub struct PffttPlugins;

impl Plugin for PffttPlugins {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_ground);
    }
}

pub fn setup (

) {

}