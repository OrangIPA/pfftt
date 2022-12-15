use bevy::prelude::*;
use pfftt::{PffttPlugins, setup, level::{PlayerSpawned, SpawnPosition}};
fn main() {
    App::new()
        .insert_resource(PlayerSpawned(false))
        .insert_resource(SpawnPosition(None))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PffttPlugins)
        .add_startup_system(setup)
        .run();
}
