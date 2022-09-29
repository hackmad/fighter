//! Player

use crate::common::*;
use crate::GROUND_Y;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::{collide_aabb, MaterialMesh2dBundle};
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

/// Animation frame used to determine collisions for player 1's attack.
const P1_ATTACK_FRAME: usize = 4;

/// Animation frame used to determine collisions for player 2's attack.
const P2_ATTACK_FRAME: usize = 2;

/// Handles the player mechanics.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(input_system)
            .add_system(movement_system)
            .add_system(collision_system)
            .add_system(animation_system);
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
#[derive(Copy, Clone, Debug, PartialEq)]
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
        "Martial Hero/Sprites/SpriteSheet.png",
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
        "Martial Hero 2/Sprites/SpriteSheet.png",
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
        if keyboard_input.pressed(keys.attack) && current_state.0 != State::Attacking {
            previous_state.set_from_current(&current_state);
            current_state.set_state(State::Attacking);
        }
    }
}

/// Handle player movement based on velocity.
fn movement_system(
    mut player_query: Query<
        (
            &Number,
            &mut CurrentState,
            &mut Transform,
            &GroundY,
            &mut Velocity,
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

    for (number, mut current_state, mut transform, ground_y, mut velocity) in &mut player_query {
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

        // If player is attacking then let them finish attacking.
        if current_state.0 != State::Attacking {
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

        // Store positions for collider_boxes.
        new_velocities.insert(*number, Vec2::new(velocity.x, velocity.y));
    }

    // Move collider with same velocity as player.
    //
    // TODO: We might want to adjust collider based on state as the box tends to not be
    // aligned with animations for different states.
    for (mut collider_box, number, mut transform, ground_y) in &mut collider_box_query {
        if let Some(velocity) = new_velocities.get(number) {
            transform.translation.x += velocity.x;
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
            transform.translation.x += velocity.x;
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
        ),
        With<Player>,
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
    for (number, current_state, previous_state, current_frame) in &player_query {
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
    for (number, mut current_state, mut previous_state, _current_frame) in &mut player_query {
        let (
            opp_attack_frame,
            (opp_current_state, _opp_previous_state, opp_current_frame),
            (opp_attack_box_pos, opp_attack_box_size),
            (collider_box_pos, collider_box_size),
        ) = match number {
            Number::One => (
                P2_ATTACK_FRAME,
                players.get(&Number::Two).unwrap(),
                attack_boxes.get(&Number::Two).unwrap(),
                collider_boxes.get(number).unwrap(),
            ),
            Number::Two => (
                P1_ATTACK_FRAME,
                players.get(&Number::One).unwrap(),
                attack_boxes.get(&Number::One).unwrap(),
                collider_boxes.get(number).unwrap(),
            ),
        };

        if *opp_current_state == State::Attacking && *opp_current_frame == opp_attack_frame {
            match collide_aabb::collide(
                *opp_attack_box_pos,
                *opp_attack_box_size,
                *collider_box_pos,
                *collider_box_size,
            ) {
                Some(_) => {
                    println!("Player {:?} hit", number);
                    previous_state.0 = current_state.0;
                    current_state.0 = State::TakingHit;
                }
                None => {
                    println!("Player {:?} no hit", number);
                }
            }
        }
    }
}

/// Animate the player sprite.
fn animation_system(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Number,
            &mut CurrentState,
            &PreviousState,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut CurrentFrame,
        ),
        With<Player>,
    >,
) {
    for (
        player_number,
        mut current_state,
        previous_state,
        mut timer,
        mut sprite,
        mut current_frame,
    ) in &mut player_query
    {
        timer.tick(time.delta());
        if timer.just_finished() {
            let (frame, looped) = next_frame(player_number, current_state.0, sprite.index);

            if looped
                && (current_state.0 == State::Attacking || current_state.0 == State::TakingHit)
            {
                // Atack/Taking Hit finished. Start previous state animation again.
                current_state.set_from_previous(previous_state);
                let (frame, _) = next_frame(player_number, current_state.0, 0);
                sprite.index = frame;
                current_frame.0 = frame;
            } else {
                sprite.index = frame;
                current_frame.0 = frame;
            }
        }
    }
}

/// Gets next animation frame for player.
fn next_frame(player_number: &Number, state: State, frame: usize) -> (usize, bool) {
    match player_number {
        Number::One => match state {
            State::Attacking => next_player_sprite_frame(frame, 0, 5),
            State::Dying => next_player_sprite_frame(frame, 16, 21),
            State::Falling => next_player_sprite_frame(frame, 24, 25),
            State::Idling => next_player_sprite_frame(frame, 32, 39),
            State::Jumping => next_player_sprite_frame(frame, 40, 41),
            State::Running => next_player_sprite_frame(frame, 48, 55),
            State::TakingHit => next_player_sprite_frame(frame, 64, 67),
        },
        Number::Two => match state {
            State::Attacking => next_player_sprite_frame(frame, 0, 3),
            State::Dying => next_player_sprite_frame(frame, 16, 22),
            State::Falling => next_player_sprite_frame(frame, 24, 25),
            State::Idling => next_player_sprite_frame(frame, 32, 35),
            State::Jumping => next_player_sprite_frame(frame, 40, 41),
            State::Running => next_player_sprite_frame(frame, 48, 55),
            State::TakingHit => next_player_sprite_frame(frame, 56, 58),
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
