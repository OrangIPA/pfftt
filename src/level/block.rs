use bevy::prelude::*;

use crate::SCALE;
use crate::ground::Block;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    Air,
    Dirt,
}

impl BlockType {
    pub fn spawn(
        &self,
        pos: (f32, f32),
        mut surrounding_blocks: [Option<BlockType>; 4], // Start from top, clockwise
        commands: &mut Commands,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &Res<AssetServer>,
    ){
        match self {
            BlockType::Air => (),
            BlockType::Dirt => {
                let texture_handle = asset_server.load("dirt.png");
                let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 4, 4, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                for block in surrounding_blocks.iter_mut() {
                    match block {
                        Some(s)
                        if *s != BlockType::Dirt => {
                            *block = None;
                        },
                        Some(_) => (),
                        None => (),
                    }
                }

                let index: usize = match surrounding_blocks {
                    [None, Some(_), None, Some(_)] => 0,
                    [None, Some(_), None, None] => 1,
                    [None, None, None, Some(_)] => 2,
                    [None, None, None, None] => 3,
                    [None, Some(_), Some(_), None] => 4,
                    [None, Some(_), Some(_), Some(_)] => 5,
                    [None, None, Some(_), Some(_)] => 6,
                    [None, None, Some(_), None] => 7,
                    [Some(_), Some(_), Some(_), None] => 8,
                    [Some(_), Some(_), Some(_),Some(_)] => 9,
                    [Some(_), None, Some(_), Some(_)] => 10,
                    [Some(_), None, Some(_), None] => 11,
                    [Some(_), Some(_), None, None] => 12,
                    [Some(_), Some(_), None, Some(_)] => 13,
                    [Some(_), None, None, Some(_)] => 14,
                    [Some(_), None, None, None] => 15
                };

                commands.spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(pos.0 * 24. * SCALE, pos.1 * 24. * SCALE, 0.),
                        scale: Vec3::new(SCALE, SCALE, 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Block);
            },
        };
    }
}
