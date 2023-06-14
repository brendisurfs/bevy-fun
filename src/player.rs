use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    if let Ok((player, mut timer, mut sprite)) = query.get_single_mut() {
        let range = match player.rotation {
            PlayerRotation::Down => AnimationIndices { first: 0, last: 5 },
            PlayerRotation::Up => AnimationIndices {
                first: 12,
                last: 17,
            },
            PlayerRotation::Left => AnimationIndices { first: 6, last: 11 },
            PlayerRotation::Right => AnimationIndices {
                first: 18,
                last: 23,
            },
        };
        timer.tick(time.delta());

        if timer.just_finished() {
            // adjust the sprite index to the next sprite in the animation range.
            sprite.index =
                if sprite.index >= range.last || sprite.index < range.first {
                    range.first
                } else {
                    sprite.index + 1
                }
        }
    }
}

// the direction that the player is facing.
pub enum PlayerRotation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub rotation: PlayerRotation,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().expect("no window found");
    let texture_handle = asset_server.load("2.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(96.0, 96.0),
        6,
        8,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 47 };

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            rotation: PlayerRotation::Down,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if direction.length() > 0.0 {
            direction = direction.normalize_or_zero();
        }

        let shift =
            keyboard_input.any_pressed([KeyCode::LShift, KeyCode::RShift]);
        let speed = if shift { 2.0 } else { 1.0 };

        // Directional movement
        if keyboard_input.pressed(KeyCode::A) {
            player.rotation = PlayerRotation::Left;
            direction += Vec3::new(-1.0 * speed, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            player.rotation = PlayerRotation::Right;
            direction += Vec3::new(1.0 * speed, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            player.rotation = PlayerRotation::Up;
            direction += Vec3::new(0.0, 1.0 * speed, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            player.rotation = PlayerRotation::Down;
            direction += Vec3::new(0.0, -1.0 * speed, 0.0);
        }

        // // Diagonal movement (jump)
        // if keyboard_input.pressed(KeyCode::Space) {
        //     direction += Vec3::new(0.0, 1.0, 0.0);
        // }

        transform.translation +=
            direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().expect("no window found");

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // clamp x
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // clamp y
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
