#![allow(dead_code)]
use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use std::ops::Add;

/// Gravity strength.
const GRAVITY: f32 = 0.7;

/// Scaling factor for background sprite.
const BG_SCALE: f32 = 3.2;

/// Ground location along y-axis.
const GROUND_Y: f32 = -66.0 * BG_SCALE;

/// Scaling factor for shop sprite.
const SHOP_SCALE: f32 = 2.85;

/// Scaling factor for player sprite.
const PLAYER_SCALE: f32 = 2.75;

/// Initial velocity for player jumps.
const JUMP_VELOCITY: f32 = 20.0;

/// Velocity for horizontal player movement.
const HORIZ_VELOCITY: f32 = 5.0;

/// Window width.
const WINDOW_WIDTH: f32 = 1024.0;

/// Window height.
const WINDOW_HEIGHT: f32 = 576.0;

fn main() {
    // Create a new application and launch game.
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Fighter".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(shop_animation_system)
        .add_system(player_animation_system)
        .add_system(player_input_system)
        .add_system(player_movement_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

/// Player number.
#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerNumber {
    One,
    Two,
}

/// Player states.
#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerState {
    Attacking,
    Dying,
    Falling,
    Idling,
    Jumping,
    Running,
    TakingHit,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self::Idling
    }
}

/// Represents the player.
#[derive(Component)]
struct Player {
    number: PlayerNumber,
}
impl Player {
    fn new(number: PlayerNumber) -> Self {
        Self { number }
    }
}

/// Represents player's current state.
#[derive(Component, Default, Deref, DerefMut)]
struct CurrentState(PlayerState);
impl CurrentState {
    fn set_state(&mut self, state: PlayerState) {
        self.0 = state;
    }
    fn set_from_previous(&mut self, state: &PreviousState) {
        self.0 = state.0;
    }
}

/// Represents player's previous state.
#[derive(Component, Default, Deref, DerefMut)]
struct PreviousState(PlayerState);
impl PreviousState {
    fn set_state(&mut self, state: PlayerState) {
        self.0 = state;
    }
    fn set_from_current(&mut self, state: &CurrentState) {
        self.0 = state.0;
    }
}

/// Represents a velocity.
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

/// Used to adjust sprite's y location based on vertical padding in sprite sheet.
#[derive(Component)]
struct GroundY(f32);

/// Represents player action keys.
#[derive(Component)]
struct Keys {
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
    attack: KeyCode,
}

/// Represents the shop sprite.
#[derive(Component)]
struct Shop;

/// Setup the game.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Camera.
    commands.spawn_bundle(Camera2dBundle::default());

    // Background sprite.
    let bg_image: Handle<Image> =
        asset_server.load("oak_woods_v1.0/background/background_composite.png");
    commands.spawn_bundle(SpriteBundle {
        texture: bg_image,
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::new(BG_SCALE, BG_SCALE, 1.0)),
        ..default()
    });

    // Shop sprite sheet.
    let shop_atlas_handle = load_shop_sprite(
        &asset_server,
        &mut texture_atlases,
        "oak_woods_v1.0/decorations/shop_anim.png",
        Vec2::new(708.0, 128.0),
        6,
    );
    commands
        .spawn()
        .insert(Shop)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: shop_atlas_handle,
            transform: Transform::from_xyz(280.0, -28.5, 0.1)
                .with_scale(Vec3::new(SHOP_SCALE, SHOP_SCALE, 1.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    // Player 1.
    let player_atlas_handle = load_player_sprite(
        &asset_server,
        &mut texture_atlases,
        "Martial Hero/Sprites/SpriteSheet.png",
        Vec2::new(1600.0, 1800.0),
        8,
        9,
    );

    // Adjust player feet (Height=200, y-feet=122 => y-center=100 => y-offset=22).
    let pos = Vec3::new(-300.0, GROUND_Y, 0.2);
    let sprite_pos = pos.add(Vec3::new(0.0, 22.0 * PLAYER_SCALE, 0.0));

    commands
        .spawn()
        .insert(Player::new(PlayerNumber::One))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(sprite_pos.y))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 32, // Idling frame start. Avoids starting at Attacking frame.
                ..default()
            },
            transform: Transform::from_translation(sprite_pos).with_scale(Vec3::new(
                PLAYER_SCALE,
                PLAYER_SCALE,
                1.0,
            )),
            ..default()
        })
        .insert(Keys {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::W,
            attack: KeyCode::S,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    // Player 2.
    let player_atlas_handle = load_player_sprite(
        &asset_server,
        &mut texture_atlases,
        "Martial Hero 2/Sprites/SpriteSheet.png",
        Vec2::new(1600.0, 1600.0),
        8,
        8,
    );

    // Adjust player feet. (Height=200, y-feet=128 => y-center=100 => y-offset=28).
    let pos = Vec3::new(300.0, GROUND_Y, 0.2);
    let sprite_pos = pos.add(Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.0));

    commands
        .spawn()
        .insert(Player::new(PlayerNumber::Two))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(sprite_pos.y))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 32, // Idling frame start. Avoids starting at Attacking frame.
                ..default()
            },
            transform: Transform::from_translation(sprite_pos).with_scale(Vec3::new(
                -PLAYER_SCALE,
                PLAYER_SCALE,
                1.0,
            )),
            ..default()
        })
        .insert(Keys {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            attack: KeyCode::Down,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

/// Load an animated sprite sheet for the shop.
fn load_shop_sprite(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    path: &str,
    size: Vec2,
    frames: usize,
) -> Handle<TextureAtlas> {
    let image: Handle<Image> = asset_server.load(path);
    let texture_atlas =
        TextureAtlas::from_grid(image, Vec2::new(size.x / frames as f32, size.y), frames, 1);
    texture_atlases.add(texture_atlas)
}

/// Load an animated sprite sheet for the player.
fn load_player_sprite(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    path: &str,
    size: Vec2,
    x_frames: usize,
    y_frames: usize,
) -> Handle<TextureAtlas> {
    let image: Handle<Image> = asset_server.load(path);
    let texture_atlas = TextureAtlas::from_grid(
        image,
        Vec2::new(size.x / x_frames as f32, size.y / y_frames as f32),
        x_frames,
        y_frames,
    );
    texture_atlases.add(texture_atlas)
}

/// Timer for animating sprites.
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

/// Animate the shop sprite.
fn shop_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Shop,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_shop, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

/// Handle play input.
fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player,
        &mut CurrentState,
        &mut PreviousState,
        &Keys,
        &Transform,
        &GroundY,
        &mut Velocity,
    )>,
) {
    for (_player, mut current_state, mut previous_state, keys, transform, ground_y, mut velocity) in
        query.iter_mut()
    {
        // Move left as long as left key is pressed.
        if keyboard_input.pressed(keys.left) {
            velocity.x = -HORIZ_VELOCITY;
        } else if keyboard_input.just_released(keys.left) {
            velocity.x = 0.0;
        }

        // Move right as long as right key is pressed.
        if keyboard_input.pressed(keys.right) {
            velocity.x = HORIZ_VELOCITY;
        } else if keyboard_input.just_released(keys.right) {
            velocity.x = 0.0;
        }

        // Jump gives an initial upward velocity which will be adjusted based on GRAVITY.
        if keyboard_input.pressed(keys.jump) {
            if transform.translation.y == ground_y.0 {
                velocity.y = JUMP_VELOCITY;
            }
        }

        // Attack.
        if keyboard_input.pressed(keys.attack) && current_state.0 != PlayerState::Attacking {
            previous_state.set_from_current(&current_state);
            current_state.set_state(PlayerState::Attacking);
        }
    }
}

/// Handle player movement based on velocity.
fn player_movement_system(
    mut query: Query<(
        &Player,
        &mut CurrentState,
        &mut Transform,
        &GroundY,
        &mut Velocity,
    )>,
) {
    for (_player, mut current_state, mut transform, ground_y, mut velocity) in &mut query {
        // Handle movement.
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        if transform.translation.y > ground_y.0 {
            // Player is in the air keep decreasing velocity.
            velocity.y -= GRAVITY;
        } else if transform.translation.y <= ground_y.0 {
            // Player has hit the ground. Reset velocity and position.
            transform.translation.y = ground_y.0;
            velocity.y = 0.0;
        }

        // Take care of player state changes.
        if current_state.0 == PlayerState::Attacking {
            // Let player finish attacking.
            return;
        }

        // Switch to jumping/falling/run/idling state.
        if transform.translation.y > ground_y.0 {
            if velocity.y > 0.0 {
                current_state.0 = PlayerState::Jumping;
            } else {
                current_state.0 = PlayerState::Falling;
            }
        } else if velocity.x != 0.0 {
            current_state.0 = PlayerState::Running;
        } else {
            current_state.0 = PlayerState::Idling;
        }
    }
}

/// Animate the player sprite.
fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &mut CurrentState,
        &PreviousState,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (player, mut current_state, previous_state, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let (frame, looped) = player_next_frame(player.number, current_state.0, sprite.index);

            if current_state.0 == PlayerState::Attacking && looped {
                // Atack finished. Start previous state animation again.
                current_state.set_from_previous(previous_state);
                let (frame, _) = player_next_frame(player.number, current_state.0, 0);
                sprite.index = frame;
            } else {
                sprite.index = frame;
            }
        }
    }
}

/// Gets next animation frame for player.
fn player_next_frame(
    player_number: PlayerNumber,
    state: PlayerState,
    frame: usize,
) -> (usize, bool) {
    match player_number {
        PlayerNumber::One => match state {
            PlayerState::Attacking => next_player_sprite_frame(frame, 0, 5),
            PlayerState::Dying => next_player_sprite_frame(frame, 16, 21),
            PlayerState::Falling => next_player_sprite_frame(frame, 24, 25),
            PlayerState::Idling => next_player_sprite_frame(frame, 32, 39),
            PlayerState::Jumping => next_player_sprite_frame(frame, 40, 41),
            PlayerState::Running => next_player_sprite_frame(frame, 48, 55),
            PlayerState::TakingHit => next_player_sprite_frame(frame, 64, 67),
        },
        PlayerNumber::Two => match state {
            PlayerState::Attacking => next_player_sprite_frame(frame, 0, 3),
            PlayerState::Dying => next_player_sprite_frame(frame, 16, 22),
            PlayerState::Falling => next_player_sprite_frame(frame, 24, 25),
            PlayerState::Idling => next_player_sprite_frame(frame, 32, 35),
            PlayerState::Jumping => next_player_sprite_frame(frame, 40, 41),
            PlayerState::Running => next_player_sprite_frame(frame, 48, 55),
            PlayerState::TakingHit => next_player_sprite_frame(frame, 56, 58),
        },
    }
}

/// Returns the next frame for player sprite.
fn next_player_sprite_frame(mut current: usize, min: usize, max: usize) -> (usize, bool) {
    if current < min || current > max {
        // Out of bounds for current player state. Reset to min.
        (min, false)
    } else {
        current = current + 1;
        if current > max {
            (min, true)
        } else {
            (current, false)
        }
    }
}
