//! Player

use crate::common::*;
use crate::GROUND_Y;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ops::Add;

/// Scaling factor for player sprite.
const PLAYER_SCALE: f32 = 2.75;

/// Starting frame for idle animation.
const IDLE_FRAME_START: usize = 32;

/// Initial velocity for player jumps.
const JUMP_VELOCITY: f32 = 20.0;

/// Velocity for horizontal player movement.
const HORIZ_VELOCITY: f32 = 5.0;

/// Collider alpha (used for displaying collider for debugging).
const COLLIDER_ALPHA: f32 = 0.4;

/// Starting health stat.
const MAX_HEALTH: u16 = 100;

/// Animation frame used to determine collisions a player's attack.
const ATTACK_FRAMES: [usize; 2] = [4, 2];

/// Attack damage of player. Player one has slow powerful attack while player two
/// has quick weaker attack (based on number of frames of animation).
const ATTACK_DAMAGES: [u16; 2] = [10_u16, 8_u16];

lazy_static! {
    /// Frame ranges for player states (min, max).
    static ref FRAMES: [HashMap<State, (usize, usize)>; 2] = {
        let mut p1 = HashMap::new();
        p1.insert(State::Attacking, (0, 5));
        p1.insert(State::Dying, (16, 21));
        p1.insert(State::Falling, (24, 25));
        p1.insert(State::Idling, (32, 39));
        p1.insert(State::Jumping, (40, 41));
        p1.insert(State::Running, (48, 55));
        p1.insert(State::TakingHit, (64, 67));

        let mut p2 = HashMap::new();
        p2.insert(State::Attacking, (0, 3));
        p2.insert(State::Dying, (16, 22));
        p2.insert(State::Falling, (24, 25));
        p2.insert(State::Idling, (32, 35));
        p2.insert(State::Jumping, (40, 41));
        p2.insert(State::Running, (48, 55));
        p2.insert(State::TakingHit, (56, 58));

        [p1, p2]
    };
}

/// Handles the player mechanics.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::new()
                .with_system(collision_system)
                .with_system(input_system.before(collision_system))
                .with_system(movement_system.before(collision_system))
                .with_system(animation_system),
        );
    }
}

/// Represents the player.
#[derive(Component, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    One,
    Two,
}
impl Player {
    fn index(&self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
    fn opponent(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

/// Player states.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum State {
    Attacking,
    Dying,
    Falling,
    Idling,
    Jumping,
    Running,
    TakingHit,
}
impl Default for State {
    fn default() -> Self {
        Self::Idling
    }
}

/// Represents player's current state.
#[derive(Component, Default, Deref, DerefMut)]
struct CurrentState(State);
impl CurrentState {
    fn set_state(&mut self, state: State) {
        self.0 = state;
    }
    fn set_from_previous(&mut self, state: &PreviousState) {
        self.0 = state.0;
    }
}

/// Represents player's previous state.
#[derive(Component, Default, Deref, DerefMut)]
struct PreviousState(State);
impl PreviousState {
    fn set_state(&mut self, state: State) {
        self.0 = state;
    }
    fn set_from_current(&mut self, state: &CurrentState) {
        self.0 = state.0;
    }
}

/// Represents player velocity.
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

/// Represents the bounding box for testing attack collisions.
#[derive(Component)]
struct ColliderBox;

/// Represents the attack box for testing with collider_boxes.
#[derive(Component)]
struct AttackBox;

/// Represents the current sprite index (used for determining collision).
#[derive(Component, Deref, DerefMut)]
struct CurrentFrame(usize);

/// Represents the health.
#[derive(Component, Deref, DerefMut)]
struct Health(u16);

/// Setup the players.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Player 1.
    let player_atlas_handle = load_sprite(
        &asset_server,
        &mut texture_atlases,
        "player_one.png",
        Vec2::new(1600.0, 1800.0),
        8,
        9,
    );

    // Adjust player feet (Height=200, y-feet=122 => y-center=100 => y-offset=22).
    let pos = Vec3::new(-300.0, GROUND_Y, 0.2);
    let player_pos = pos.add(Vec3::new(0.0, 22.0 * PLAYER_SCALE, 0.21));
    let collider_box_pos = Vec3::new(0.0, 15.0, 0.22);
    let collider_box_size = Vec3::new(30.0, 55.0, 1.0) * PLAYER_SCALE;
    let attack_box_pos = Vec3::new(145.0, 56.0, 0.23);
    let attack_box_size = Vec3::new(75.0, 25.0, 1.0) * PLAYER_SCALE;

    commands
        .spawn()
        .insert(Player::One)
        .insert(Health(MAX_HEALTH))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(player_pos.y))
        .insert(CurrentFrame(IDLE_FRAME_START))
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform {
                translation: player_pos,
                ..default()
            },
            ..default()
        })
        .insert(Keys {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::W,
            attack: KeyCode::S,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .with_children(|player| {
            player.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: player_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: IDLE_FRAME_START, // Idling frame start. Avoids starting at Attacking frame.
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                    ..default()
                },
                ..default()
            });

            player
                .spawn()
                .insert(ColliderBox)
                .insert(GroundY(collider_box_pos.y))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 0.0, 0.0, COLLIDER_ALPHA),
                        ..default()
                    },
                    transform: Transform {
                        translation: collider_box_pos,
                        scale: collider_box_size,
                        ..default()
                    },
                    ..default()
                });

            player
                .spawn()
                .insert(AttackBox)
                .insert(GroundY(attack_box_pos.y))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 0.0, COLLIDER_ALPHA),
                        ..default()
                    },
                    transform: Transform {
                        translation: attack_box_pos,
                        scale: attack_box_size,
                        ..default()
                    },
                    ..default()
                });
        });

    // Player 2.
    let player_atlas_handle = load_sprite(
        &asset_server,
        &mut texture_atlases,
        "player_two.png",
        Vec2::new(1600.0, 1600.0),
        8,
        8,
    );

    // Adjust player feet. (Height=200, y-feet=128 => y-center=100 => y-offset=28).
    let pos = Vec3::new(300.0, GROUND_Y, 0.2);
    let player_pos = pos.add(Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.21));
    let collider_box_pos = Vec3::new(0.0, 0.0, 0.22);
    let collider_box_size = Vec3::new(25.0, 58.0, 1.0) * PLAYER_SCALE;
    let attack_box_pos = Vec3::new(-130.0, 32.0, 0.23);
    let attack_box_size = Vec3::new(70.0, 35.0, 1.0) * PLAYER_SCALE;

    commands
        .spawn()
        .insert(Player::Two)
        .insert(Health(MAX_HEALTH))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(player_pos.y))
        .insert(CurrentFrame(IDLE_FRAME_START))
        .insert_bundle(SpatialBundle {
            visibility: Visibility { is_visible: true },
            transform: Transform {
                translation: player_pos,
                ..default()
            },
            ..default()
        })
        .insert(Keys {
            left: KeyCode::Left,
            right: KeyCode::Right,
            jump: KeyCode::Up,
            attack: KeyCode::Down,
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .with_children(|player| {
            player.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: player_atlas_handle,
                sprite: TextureAtlasSprite {
                    index: IDLE_FRAME_START, // Idling frame start. Avoids starting at Attacking frame.
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(-PLAYER_SCALE, PLAYER_SCALE, 1.0),
                    ..default()
                },
                ..default()
            });

            player
                .spawn()
                .insert(ColliderBox)
                .insert(GroundY(collider_box_pos.y))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, COLLIDER_ALPHA),
                        ..default()
                    },
                    transform: Transform {
                        translation: collider_box_pos,
                        scale: collider_box_size,
                        ..default()
                    },
                    ..default()
                });

            player
                .spawn()
                .insert(AttackBox)
                .insert(GroundY(attack_box_pos.y))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 0.0, 1.0, COLLIDER_ALPHA),
                        ..default()
                    },
                    transform: Transform {
                        translation: attack_box_pos,
                        scale: attack_box_size,
                        ..default()
                    },
                    ..default()
                });
        });
}

/// Load an animated sprite sheet for the player.
fn load_sprite(
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

/// Handle play input.
fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut CurrentState,
            &mut PreviousState,
            &Keys,
            &Transform,
            &GroundY,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut current_state, mut previous_state, keys, transform, ground_y, mut velocity) in
        player_query.iter_mut()
    {
        // Don't do anything if player is dead.
        match current_state.0 {
            State::Dying => continue,
            _ => (),
        }

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

        if keyboard_input.pressed(keys.attack) {
            // If player is either attacking already or taking a hit don't allow an attack.
            match current_state.0 {
                State::Attacking | State::TakingHit => (),
                _ => {
                    previous_state.set_from_current(&current_state);
                    current_state.set_state(State::Attacking);
                }
            }
        }
    }
}

/// Handle player movement based on velocity.
fn movement_system(
    mut player_query: Query<(
        &Player,
        &mut CurrentState,
        &PreviousState,
        &mut Transform,
        &GroundY,
        &mut Velocity,
        &CurrentFrame,
        &Health,
    )>,
) {
    let mut new_velocities = [Vec2::default(); 2];
    let mut move_x = [false; 2];

    for (
        player,
        mut current_state,
        previous_state,
        mut transform,
        ground_y,
        mut velocity,
        current_frame,
        health,
    ) in &mut player_query
    {
        // Handle horizontal movement.
        let new_x = transform.translation.x + velocity.x;
        if new_x > crate::scene::SCENE_MIN_X && new_x < crate::scene::SCENE_MAX_X {
            transform.translation.x = new_x;
            move_x[player.index()] = true;
        } else {
            move_x[player.index()] = false;
        }

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

        // Check if player is dying.
        if health.0 == 0 {
            current_state.set_state(State::Dying);
        }

        match current_state.0 {
            State::Dying => {
                // Don't do anything. Game over.
            }
            State::Attacking => {
                // Let player finish attacking.
                let max_frame = FRAMES[player.index()].get(&State::Attacking).unwrap().1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::TakingHit => {
                // Let player finish taking hit.
                let max_frame = FRAMES[player.index()].get(&State::TakingHit).unwrap().1;
                if current_frame.0 == max_frame {
                    match previous_state.0 {
                        State::Attacking => {
                            // Don't resume attacking state after taking a hit.
                            // Determine state based on position/velocity.
                            if transform.translation.y > ground_y.0 {
                                if velocity.y > 0.0 {
                                    current_state.0 = State::Jumping;
                                } else {
                                    current_state.0 = State::Falling;
                                }
                            } else if velocity.x != 0.0 {
                                current_state.0 = State::Running;
                            } else {
                                current_state.0 = State::Idling;
                            }
                        }
                        _ => {
                            // Resume previous state.
                            current_state.set_from_previous(previous_state);
                        }
                    }
                }
            }
            _ => {
                // Determine state based on position/velocity.
                if transform.translation.y > ground_y.0 {
                    if velocity.y > 0.0 {
                        current_state.0 = State::Jumping;
                    } else {
                        current_state.0 = State::Falling;
                    }
                } else if velocity.x != 0.0 {
                    current_state.0 = State::Running;
                } else {
                    current_state.0 = State::Idling;
                }
            }
        }

        // Store positions for collider_boxes.
        new_velocities[player.index()] = Vec2::new(velocity.x, velocity.y);
    }
}

/// Handle collision detection.
fn collision_system(
    mut player_query: Query<(
        &Player,
        &mut CurrentState,
        &mut PreviousState,
        &CurrentFrame,
        &mut Health,
    )>,
    collider_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<ColliderBox>>,
    attack_box_query: Query<(&Parent, &GlobalTransform, &Transform), With<AttackBox>>,
) {
    // Since we need to check one player's collider with the opponent's attack_box we need to
    // load this information before running the collision detection.
    let mut players = [(State::default(), State::default(), 0_usize); 2];
    for (player, current_state, previous_state, current_frame, _health) in &player_query {
        players[player.index()] = (current_state.0, previous_state.0, current_frame.0);
    }

    let mut collider_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &collider_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        collider_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    let mut attack_boxes = [(Vec3::default(), Vec2::default()); 2];
    for (parent, gt, t) in &attack_box_query {
        let (player, _, _, _, _) = player_query.get(parent.get()).unwrap();
        attack_boxes[player.index()] = (gt.translation(), t.scale.truncate());
    }

    // Check collision detection.
    for (player, mut current_state, mut previous_state, _current_frame, mut health) in
        &mut player_query
    {
        match current_state.0 {
            State::TakingHit | State::Dying => continue,
            _ => (),
        }

        let opponent = player.opponent().index();
        let (collider_box_pos, collider_box_size) = collider_boxes[player.index()];
        let opponent_attack_frame = ATTACK_FRAMES[opponent];
        let (opponent_current_state, _opponent_previous_state, opponent_current_frame) =
            players[opponent];
        let (opponent_attack_box_pos, opponent_attack_box_size) = attack_boxes[opponent];
        let opponent_attack_damage: u16 = ATTACK_DAMAGES[opponent];

        match opponent_current_state {
            State::Attacking => {
                if opponent_current_frame == opponent_attack_frame {
                    if collide(
                        opponent_attack_box_pos,
                        opponent_attack_box_size,
                        collider_box_pos,
                        collider_box_size,
                    )
                    .is_some()
                    {
                        println!(
                            "Collision at {:?} {:?} - cb {:?} {:?}",
                            opponent_attack_box_pos,
                            opponent_attack_box_size,
                            collider_box_pos,
                            collider_box_size,
                        );

                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        let mut new_health: i16 = health.0 as i16 - opponent_attack_damage as i16;
                        if new_health < 0 {
                            new_health = 0;
                        }
                        health.0 = new_health as u16;

                        println!("Player {:?} hit. Health = {}.", player, health.0);
                    }
                }
            }
            _ => (),
        }
    }
}

/// Animate the player sprite.
fn animation_system(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Player,
            &CurrentState,
            &mut AnimationTimer,
            &mut CurrentFrame,
        ),
        With<Player>,
    >,
    mut sprite_query: Query<(&Parent, &mut TextureAtlasSprite)>,
) {
    for (parent, mut sprite) in &mut sprite_query {
        let (player, current_state, mut timer, mut current_frame) =
            player_query.get_mut(parent.get()).unwrap();
        timer.tick(time.delta());
        if timer.just_finished() {
            let (frame, _looped) = next_frame(player, current_state.0, sprite.index);
            sprite.index = frame;
            current_frame.0 = frame;
        }
    }
}

/// Gets next animation frame for player.
fn next_frame(player: &Player, state: State, current: usize) -> (usize, bool) {
    let (start, end) = FRAMES[player.index()].get(&state).unwrap();
    let (frame, looped) = next_player_sprite_frame(current, *start, *end);

    match state {
        State::Dying => {
            // Don't loop dying animation.
            if looped {
                (*end, false)
            } else {
                (frame, looped)
            }
        }
        _ => (frame, looped),
    }
}

/// Returns the next frame for player sprite.
fn next_player_sprite_frame(mut current: usize, start: usize, end: usize) -> (usize, bool) {
    if current < start || current > end {
        // Out of bounds for current player state. Reset to start.
        (start, false)
    } else {
        current = current + 1;
        if current > end {
            (start, true)
        } else {
            (current, false)
        }
    }
}
