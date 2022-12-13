use bevy::prelude::*;
use pfftt::*;
fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PffttPlugins)
        .run();
}
