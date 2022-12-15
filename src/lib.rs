mod player;
mod ground;
mod camera;
pub mod level;

use bevy::{prelude::*, app::PluginGroupBuilder};

use level::LevelPlugin;
use player::PlayerPlugin;
// use ground::GroundPlugin;
use camera::CameraPlugin;

pub const SCALE: f32 = 3.;

pub struct PffttPlugins;

impl PluginGroup for PffttPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(CameraPlugin)
            .add(LevelPlugin)
    }
}

pub fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(camera::Camera);
}
