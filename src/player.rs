use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::ground::*;
use crate::SCALE;

const MAX_SPEED: f32 = 150.;
const ACCELERATION: f32 = 70.;
const DECELERATION: f32 = 70.;
const GRAVITY: f32 = -25.;
const JUMP_SPEED: f32 = 350.;

const PLAYER_SIZE: (f32, f32) = (18., 24.);

#[derive(PartialEq)]
enum Direction {
    Positive,
    Negative,
    None,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Default)]
pub struct PlayerMovement {
    vel: Vec2,
    acc: Vec2,
    dir: Direction,
    touch_ground: bool,
    double_jump: bool,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
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
        .insert(PlayerMovement {
            acc: Vec2::new(0., GRAVITY),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
}

pub fn player_input(
    mut query: Query<&mut PlayerMovement, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for mut mov in query.iter_mut() {
        mov.dir = Direction::None;
        if input.pressed(KeyCode::D) && !input.pressed(KeyCode::A) {
            mov.dir = Direction::Positive;
        }

        if input.pressed(KeyCode::A) && !input.pressed(KeyCode::D) {
            mov.dir = Direction::Negative;
        }

        if input.just_pressed(KeyCode::Space) && mov.touch_ground {
            mov.vel.y = JUMP_SPEED;
        } else if !mov.double_jump && input.just_pressed(KeyCode::Space) {
            mov.vel.y = JUMP_SPEED;
            mov.double_jump = true;
        }
    }
}
pub fn player_update(
    mut query: Query<(&mut PlayerMovement, &mut Transform), With<Player>>,
    block_transform: Query<&Transform, (With<Block>, Without<Player>)>,
    time: Res<Time>,
) {
    for (mut mov, mut player_transform) in query.iter_mut() {

        // Set the acceleration from dir
        match mov.dir {
            Direction::Positive => mov.acc.x = ACCELERATION,
            Direction::Negative => mov.acc.x = -ACCELERATION,
            Direction::None => (),
        }

        // Deceleration
        if mov.dir == Direction::None {
            if mov.vel.x < 0. {
                mov.acc.x = DECELERATION;
                if DECELERATION < mov.vel.x {
                    mov.vel.x = 0.;
                    mov.acc.x = 0.;
                }
            }

            if mov.vel.x > 0. {
                mov.acc.x = -DECELERATION;
                if DECELERATION > mov.vel.x {
                    mov.vel.x = 0.;
                    mov.acc.x = 0.;
                }
            }
        }

        // Speed Limit
        if mov.vel.x > MAX_SPEED {
            mov.vel.x = MAX_SPEED;
        }
        if mov.vel.x < -MAX_SPEED {
            mov.vel.x = -MAX_SPEED;
        }

        // Update the velocity from acceleration
        mov.vel.x += mov.acc.x;
        mov.vel.y += mov.acc.y;

        // Update the position from velocity
        player_transform.translation.x += mov.vel.x * time.delta_seconds() * SCALE;
        player_transform.translation.y += mov.vel.y * time.delta_seconds() * SCALE;

        // Collision
        mov.touch_ground = false;
        for block_transform in block_transform.iter() {
            match collide(
                player_transform.translation,
                Vec2::new(PLAYER_SIZE.0 * SCALE, PLAYER_SIZE.1 * SCALE),
                block_transform.translation,
                Vec2::new(24. * SCALE, 24. * SCALE),
            ) {
                Some(Collision::Left) => {
                    player_transform.translation.x = block_transform.translation.x - PLAYER_SIZE.0 * SCALE;
                },
                Some(Collision::Right) => {
                    player_transform.translation.x = block_transform.translation.x + PLAYER_SIZE.0 * SCALE;
                },
                Some(Collision::Top)
                if mov.vel.y < 0. => {
                    player_transform.translation.y = block_transform.translation.y + PLAYER_SIZE.1 * SCALE;
                    mov.vel.y = 0.;
                    mov.touch_ground = true;
                    mov.double_jump = false;
                },
                Some(Collision::Bottom) => {
                    player_transform.translation.y = block_transform.translation.y - PLAYER_SIZE.1 * SCALE;
                    mov.vel.y = 0.;
                },
                _ => (),
            }
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut animation_query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    velocity_query: Query<&PlayerMovement, With<Player>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = transform_query.single_mut();
    let vel = velocity_query.single().vel.x;
    // if vel == 0. { return; }
    if vel < 0. {
        transform.scale.x = -SCALE;
    } else {
        transform.scale.x = SCALE;
    }

    for (mut timer,
        mut sprite,
        texture_atlas_handle
    ) in animation_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            if sprite.index == 0 {
                sprite.index += 1;
            }
            if vel == 0. {
                sprite.index = 0;
            }
        }
    }
}

pub fn fall_to_the_void(
    mut player: Query<(&mut Transform, &mut PlayerMovement), With<Player>>,
    input: Res<Input<KeyCode>>
) {
    let (mut trans, mut mov) = player.single_mut();
    if trans.translation.y < -100. * SCALE || (input.just_pressed(KeyCode::R)){
        trans.translation.y = -24. * SCALE;
        trans.translation.x = -24. * SCALE;
        mov.vel = Vec2::new(0., 0.);
    }
}