use bevy::prelude::*;
use pfftt::{PffttPlugins, setup};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PffttPlugins)
        .add_startup_system(setup)
        .run();
}
