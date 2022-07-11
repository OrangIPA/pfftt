use bevy::prelude::*;

use crate::SCALE;

const MAX_SPEED: f32 = 200.;
const ACCELERATION: f32 = 100.;
const DECELERATION: f32 = 160.;

#[derive(PartialEq)]
enum Direction {
    Positive, Negative, None
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

#[derive(Component)]
pub struct PlayerMovement {
    vel: Vec2,
    acc: Vec2,
    dir: Direction,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self { vel: Default::default(), acc: Default::default(), dir: Default::default() }
    }
}

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
    .insert(PlayerMovement {
        ..Default::default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
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

        if input.pressed(KeyCode::A) && !input.pressed(KeyCode::D){
            mov.dir = Direction::Negative;
        }
    }

}

pub fn player_update(
    mut query: Query<(&mut PlayerMovement, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    for (mut mov, mut transform) in query.iter_mut() {
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

        // Update the position from velocity
        transform.translation.x += mov.vel.x * time.delta_seconds() * SCALE;
    }
}

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut animation_query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    ), With<Player>>,
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

    for (mut timer, mut sprite, texture_atlas_handle) in animation_query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            if sprite.index == 0 { sprite.index += 1; }
            if vel == 0. { sprite.index = 0; }
        }
    }
}