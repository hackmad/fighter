//! Player

use crate::common::*;
use crate::GROUND_Y;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::{collide_aabb, MaterialMesh2dBundle};
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

lazy_static! {
    /// Frame ranges for player states (min, max).
    static ref FRAMES: HashMap<Number, HashMap<State, (usize, usize)>> = {
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

        let mut m = HashMap::new();
        m.insert(Number::One, p1);
        m.insert(Number::Two, p2);
        m
    };

    /// Animation frame used to determine collisions a player's attack.
    static ref ATTACK_FRAMES: HashMap<Number, usize> = {
        let mut m = HashMap::new();
        m.insert(Number::One, 4);
        m.insert(Number::Two, 2);
        m
    };

    /// Attack damage of player. Player one has slow powerful attack while player two
    /// has quick weaker attack (based on number of frames of animation).
    static ref ATTACK_DAMAGES: HashMap<Number , u16> = {
        let mut m = HashMap::new();
        m.insert(Number::One, 10_u16);
        m.insert(Number::Two, 8_u16);
        m
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
#[derive(Component)]
struct Player;

/// Player number.
#[derive(Component, Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Number {
    One,
    Two,
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
struct ColliderBox {
    pos: Vec3,
    size: Vec2,
}

/// Represents the attack box for testing with collider_boxes.
#[derive(Component)]
struct AttackBox {
    pos: Vec3,
    size: Vec2,
}

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // NOTE: Need separate entity to display collider. If we add multiple SpriteBundle,
    // SpriteSheetBundle, Mesh, etc that each have Transforms, it messes up the movement system
    // as all transforms trample over each other.

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
    let sprite_pos = pos.add(Vec3::new(0.0, 22.0 * PLAYER_SCALE, 0.21));
    let collider_box_pos = pos.add(Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.22));
    let collider_box_size = Vec2::new(30.0, 55.0) * PLAYER_SCALE;
    let attack_box_pos = pos.add(Vec3::new(37.0 * PLAYER_SCALE, 50.0 * PLAYER_SCALE, 0.23));
    let attack_box_size = Vec2::new(105.0, 25.0) * PLAYER_SCALE;

    commands
        .spawn()
        .insert(Player)
        .insert(Number::One)
        .insert(Health(MAX_HEALTH))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(sprite_pos.y))
        .insert(CurrentFrame(IDLE_FRAME_START))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: IDLE_FRAME_START, // Idling frame start. Avoids starting at Attacking frame.
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

    commands
        .spawn()
        .insert(ColliderBox {
            pos: collider_box_pos,
            size: collider_box_size,
        })
        .insert(Number::One)
        .insert(GroundY(collider_box_pos.y))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(collider_box_size.x, collider_box_size.y, 0.1).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgba(
                1.0,
                0.0,
                0.0,
                COLLIDER_ALPHA,
            ))),
            transform: Transform::from_translation(collider_box_pos),
            ..default()
        });

    commands
        .spawn()
        .insert(AttackBox {
            pos: attack_box_pos,
            size: attack_box_size,
        })
        .insert(Number::One)
        .insert(GroundY(attack_box_pos.y))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(attack_box_size.x, attack_box_size.y, 0.1).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgba(
                1.0,
                1.0,
                0.0,
                COLLIDER_ALPHA,
            ))),
            transform: Transform::from_translation(attack_box_pos),
            ..default()
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
    let sprite_pos = pos.add(Vec3::new(0.0, 28.0 * PLAYER_SCALE, 0.21));
    let collider_box_pos = pos.add(Vec3::new(-3.0 * PLAYER_SCALE, 29.0 * PLAYER_SCALE, 0.22));
    let collider_box_size = Vec2::new(25.0, 58.0) * PLAYER_SCALE;
    let attack_box_pos = pos.add(Vec3::new(-37.0 * PLAYER_SCALE, 40.0 * PLAYER_SCALE, 0.23));
    let attack_box_size = Vec2::new(95.0, 35.0) * PLAYER_SCALE;

    commands
        .spawn()
        .insert(Player)
        .insert(Number::Two)
        .insert(Health(MAX_HEALTH))
        .insert(CurrentState::default())
        .insert(PreviousState::default())
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(GroundY(sprite_pos.y))
        .insert(CurrentFrame(IDLE_FRAME_START))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: player_atlas_handle,
            sprite: TextureAtlasSprite {
                index: IDLE_FRAME_START, // Idling frame start. Avoids starting at Attacking frame.
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

    commands
        .spawn()
        .insert(ColliderBox {
            pos: collider_box_pos,
            size: collider_box_size,
        })
        .insert(Number::Two)
        .insert(GroundY(collider_box_pos.y))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(collider_box_size.x, collider_box_size.y, 0.1).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgba(
                0.0,
                0.0,
                1.0,
                COLLIDER_ALPHA,
            ))),
            transform: Transform::from_translation(collider_box_pos),
            ..default()
        });

    commands
        .spawn()
        .insert(AttackBox {
            pos: attack_box_pos,
            size: attack_box_size,
        })
        .insert(Number::Two)
        .insert(GroundY(attack_box_pos.y))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(attack_box_size.x, attack_box_size.y, 0.1).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgba(
                0.0,
                1.0,
                1.0,
                COLLIDER_ALPHA,
            ))),
            transform: Transform::from_translation(attack_box_pos),
            ..default()
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
    mut player_query: Query<
        (
            &Number,
            &mut CurrentState,
            &PreviousState,
            &mut Transform,
            &GroundY,
            &mut Velocity,
            &CurrentFrame,
            &Health,
        ),
        (With<Player>, Without<ColliderBox>, Without<AttackBox>),
    >,
    mut collider_box_query: Query<
        (&mut ColliderBox, &Number, &mut Transform, &GroundY),
        (With<ColliderBox>, Without<Player>, Without<AttackBox>),
    >,
    mut attack_box_query: Query<
        (&mut AttackBox, &Number, &mut Transform, &GroundY),
        (With<AttackBox>, Without<Player>, Without<ColliderBox>),
    >,
) {
    let mut new_velocities = HashMap::new();
    let mut move_x = HashMap::new();

    for (
        number,
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
            move_x.insert(number, true);
        } else {
            move_x.insert(number, false);
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
                let max_frame = FRAMES
                    .get(number)
                    .unwrap()
                    .get(&State::Attacking)
                    .unwrap()
                    .1;
                if current_frame.0 == max_frame {
                    current_state.set_from_previous(previous_state);
                }
            }
            State::TakingHit => {
                // Let player finish taking hit.
                let max_frame = FRAMES
                    .get(number)
                    .unwrap()
                    .get(&State::TakingHit)
                    .unwrap()
                    .1;
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
        new_velocities.insert(*number, Vec2::new(velocity.x, velocity.y));
    }

    // Move collider with same velocity as player.
    //
    // TODO: We might want to adjust collider based on state as the box tends to not be
    // aligned with animations for different states.
    for (mut collider_box, number, mut transform, ground_y) in &mut collider_box_query {
        if let Some(velocity) = new_velocities.get(number) {
            if *move_x.get(number).unwrap() {
                transform.translation.x += velocity.x;
            }

            transform.translation.y += velocity.y;
            if transform.translation.y <= ground_y.0 {
                // Player has hit the ground. Reset velocity and position.
                transform.translation.y = ground_y.0;
            }

            collider_box.pos = transform.translation;
        }
    }

    // Move attack box with same velocity as player.
    for (mut attack_box, number, mut transform, ground_y) in &mut attack_box_query {
        if let Some(velocity) = new_velocities.get(number) {
            if *move_x.get(number).unwrap() {
                transform.translation.x += velocity.x;
            }

            transform.translation.y += velocity.y;
            if transform.translation.y <= ground_y.0 {
                // Player has hit the ground. Reset velocity and position.
                transform.translation.y = ground_y.0;
            }

            attack_box.pos = transform.translation;
        }
    }
}

/// Handle collision detection.
fn collision_system(
    mut player_query: Query<
        (
            &Number,
            &mut CurrentState,
            &mut PreviousState,
            &CurrentFrame,
            &mut Health,
        ),
        (With<Player>, Without<ColliderBox>, Without<AttackBox>),
    >,
    collider_box_query: Query<
        (&ColliderBox, &Number),
        (With<ColliderBox>, Without<Player>, Without<AttackBox>),
    >,
    attack_box_query: Query<
        (&AttackBox, &Number),
        (With<AttackBox>, Without<Player>, Without<ColliderBox>),
    >,
) {
    // Since we need to check one player's collider with the opponent's attack_box we need to
    // load this information before running the collision detection.
    let mut players = HashMap::new();
    for (number, current_state, previous_state, current_frame, _health) in &player_query {
        players.insert(
            *number,
            (current_state.0, previous_state.0, current_frame.0),
        );
    }

    let mut collider_boxes = HashMap::new();
    for (collider_box, number) in &collider_box_query {
        collider_boxes.insert(*number, (collider_box.pos, collider_box.size));
    }

    let mut attack_boxes = HashMap::new();
    for (attack_box, number) in &attack_box_query {
        attack_boxes.insert(*number, (attack_box.pos, attack_box.size));
    }

    // Check collision detection.
    for (number, mut current_state, mut previous_state, _current_frame, mut health) in
        &mut player_query
    {
        match current_state.0 {
            State::TakingHit | State::Dying => continue,
            _ => (),
        }

        let (
            (collider_box_pos, collider_box_size),
            opponent_attack_frame,
            (opponent_current_state, _opponent_previous_state, opponent_current_frame),
            (opponent_attack_box_pos, opponent_attack_box_size),
            opponent_attack_damage,
        ) = match number {
            Number::One => (
                collider_boxes.get(number).unwrap(),
                ATTACK_FRAMES.get(&Number::Two).unwrap(),
                players.get(&Number::Two).unwrap(),
                attack_boxes.get(&Number::Two).unwrap(),
                ATTACK_DAMAGES.get(&Number::Two).unwrap(),
            ),
            Number::Two => (
                collider_boxes.get(number).unwrap(),
                ATTACK_FRAMES.get(&Number::One).unwrap(),
                players.get(&Number::One).unwrap(),
                attack_boxes.get(&Number::One).unwrap(),
                ATTACK_DAMAGES.get(&Number::One).unwrap(),
            ),
        };

        match opponent_current_state {
            State::Attacking => {
                if *opponent_current_frame == *opponent_attack_frame {
                    if collide_aabb::collide(
                        *opponent_attack_box_pos,
                        *opponent_attack_box_size,
                        *collider_box_pos,
                        *collider_box_size,
                    )
                    .is_some()
                    {
                        previous_state.set_state(current_state.0);
                        current_state.set_state(State::TakingHit);

                        // Just in case damage is not a nice divisior of MAX_HEALTH.
                        let mut new_health: i16 = health.0 as i16 - *opponent_attack_damage as i16;
                        if new_health < 0 {
                            new_health = 0;
                        }
                        health.0 = new_health as u16;

                        println!("Player {:?} hit. Health = {}.", number, health.0);
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
            &Number,
            &CurrentState,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut CurrentFrame,
        ),
        With<Player>,
    >,
) {
    for (number, current_state, mut timer, mut sprite, mut current_frame) in &mut player_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let (frame, _looped) = next_frame(number, current_state.0, sprite.index);
            sprite.index = frame;
            current_frame.0 = frame;
        }
    }
}

/// Gets next animation frame for player.
fn next_frame(number: &Number, state: State, current: usize) -> (usize, bool) {
    let (start, end) = FRAMES.get(number).unwrap().get(&state).unwrap();
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
