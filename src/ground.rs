use bevy::prelude::*;

pub fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dirt.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for i in 0..11 {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(i as f32 * 24. * 3., -24. * 3. * 2., 0.),
                scale: Vec3::new(3., 3., 0.),
                ..Default::default()
            },
            ..Default::default()
        });

        if i == 0 { continue; }

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-i as f32 * 24. * 3., -24. * 3. * 2., 0.),
                scale: Vec3::new(3., 3., 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}