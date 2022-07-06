use bevy::prelude::*;
use pfftt::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PffttPlugins)
        .run();
}
