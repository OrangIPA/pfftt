use bevy::prelude::*;

use crate::SCALE;

#[derive(Component)]
pub struct Block;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground);
    }
}

pub fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dirt.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for i in 0..100 {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(i as f32 * 24. * SCALE, -24. * SCALE * 2., 0.),
                    scale: Vec3::new(SCALE as f32, SCALE, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);

        if i == 0 {
            continue;
        }

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(-i as f32 * 24. * SCALE, -24. * SCALE * 2., 0.),
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);
    }
    commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 3,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(5 as f32 * 24. * SCALE, -24. * SCALE, 0.),
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);
}
