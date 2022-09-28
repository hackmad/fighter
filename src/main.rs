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

/// Player number
enum PlayerNumber {
    One,
    Two,
}

/// Player state
enum PlayerState {
    Attacking,
    Dying,
    Falling,
    Idling,
    Jumping,
    Running,
    TakingHit,
}

/// Represents the player.
#[derive(Component)]
struct Player {
    number: PlayerNumber,
    state: PlayerState,
}
impl Player {
    fn new(number: PlayerNumber) -> Self {
        Self {
            number,
            state: PlayerState::Idling,
        }
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

/// Returns the next frame for player sprite.
fn next_player_sprite_frame(mut current: usize, min: usize, max: usize) -> usize {
    if current < min || current > max {
        // Out of bounds for current player state. Reset to min.
        min
    } else {
        current = current + 1;
        if current > max {
            min
        } else {
            current
        }
    }
}

/// Animate the player sprite.
fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (player, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = match player.number {
                PlayerNumber::One => match player.state {
                    PlayerState::Attacking => next_player_sprite_frame(sprite.index, 0, 5),
                    PlayerState::Dying => next_player_sprite_frame(sprite.index, 16, 21),
                    PlayerState::Falling => next_player_sprite_frame(sprite.index, 24, 25),
                    PlayerState::Idling => next_player_sprite_frame(sprite.index, 32, 39),
                    PlayerState::Jumping => next_player_sprite_frame(sprite.index, 40, 41),
                    PlayerState::Running => next_player_sprite_frame(sprite.index, 48, 55),
                    PlayerState::TakingHit => next_player_sprite_frame(sprite.index, 64, 67),
                },
                PlayerNumber::Two => match player.state {
                    PlayerState::Attacking => next_player_sprite_frame(sprite.index, 0, 3),
                    PlayerState::Dying => next_player_sprite_frame(sprite.index, 16, 22),
                    PlayerState::Falling => next_player_sprite_frame(sprite.index, 24, 25),
                    PlayerState::Idling => next_player_sprite_frame(sprite.index, 32, 35),
                    PlayerState::Jumping => next_player_sprite_frame(sprite.index, 40, 41),
                    PlayerState::Running => next_player_sprite_frame(sprite.index, 48, 55),
                    PlayerState::TakingHit => next_player_sprite_frame(sprite.index, 56, 58),
                },
            };
        }
    }
}

/// Handle play input.
fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Keys, &Transform, &GroundY, &mut Velocity)>,
) {
    for (_player, keys, transform, ground_y, mut velocity) in query.iter_mut() {
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
    }
}

/// Handle player movement based on velocity.
fn player_movement_system(
    mut query: Query<(&mut Player, &mut Transform, &GroundY, &mut Velocity)>,
) {
    for (mut player, mut transform, ground_y, mut velocity) in &mut query {
        // Handle horizontal movement.
        transform.translation.x += velocity.x;

        // Handle vertical movement.
        transform.translation.y += velocity.y;

        if transform.translation.y > ground_y.0 {
            // Player is in the air keep decreasing velocity.
            velocity.y -= GRAVITY;
        } else if transform.translation.y <= ground_y.0 {
            // Player has hit the ground. Reset velocity and position.
            transform.translation.y = ground_y.0;
            velocity.y = 0.0;
        }

        if transform.translation.y > ground_y.0 {
            if velocity.y > 0.0 {
                player.state = PlayerState::Jumping;
            } else {
                player.state = PlayerState::Falling;
            }
        } else if velocity.x != 0.0 {
            player.state = PlayerState::Running;
        } else {
            player.state = PlayerState::Idling;
        }
    }
}
