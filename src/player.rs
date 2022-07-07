use bevy::prelude::*;

use crate::SCALE;

const MAX_SPEED: f32 = 350.;
const ACCELERATION: f32 = 60.;
const DECELERATION: f32 = 70.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Velocity(f32, f32);

#[derive(Component)]
pub struct Acceleration(f32, f32);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0., -24. * SCALE, 0.),
            scale: Vec3::new(SCALE, SCALE, 0.),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            index: 0,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity(0., 0.))
    .insert(Acceleration(0., 0.));
}
pub fn player_input(
    mut query: Query<(&mut Acceleration, &mut Velocity), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut acc, mut vel) in query.iter_mut() {
        // Decelerate player if neither 'a' or 'd' is pressed
        if !input.pressed(KeyCode::D) && !input.pressed(KeyCode::A) {

            // Decelerate player if moving to the right
            if vel.0 > 0. {
                vel.0 = -DECELERATION;
                if vel.0 < 0. {
                    vel.0 = 0.;
                    acc.0 = 0.;
                }
                continue;
            }
            
            // Decelerate player if moving to the left 
            if vel.0 < 0. {
                vel.0 = DECELERATION;
                if vel.0 > 0. {
                    vel.0 = 0.;
                    acc.0 = 0.;
                }
                continue;
            }
        }

        // Accelerate player to the right if 'D' key is pressed
        if input.pressed(KeyCode::D) && !input.pressed(KeyCode::A) {
            acc.0 = ACCELERATION;
        }

        // Accelerate player to the left if 'a' key is pressed
        if input.pressed(KeyCode::A) {
            acc.0 = -ACCELERATION;
        }
    }
}

pub fn player_update(
    mut query: Query<(&Acceleration, &mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    for (acc, mut vel, mut transform) in query.iter_mut() {
        vel.0 += acc.0;

        if vel.0 > MAX_SPEED { vel.0 = MAX_SPEED; }
        if vel.0 < -MAX_SPEED { vel.0 = -MAX_SPEED; }

        transform.translation.x += vel.0 * time.delta_seconds();
    }
}