//! Scene

use crate::{common::*, GameAssets, GameState};
use bevy::{app::Plugin, prelude::*};

/// Scaling factor for background sprite.
const BG_SCALE: f32 = 3.2;

/// Scaling factor for shop sprite.
const SHOP_SCALE: f32 = 2.85;

/// Ground location along y-axis.
pub(crate) const GROUND_Y: f32 = -66.0 * BG_SCALE;

/// Scene minimum x bounds
pub(crate) const SCENE_MIN_X: f32 = -WINDOW_WIDTH as f32 / 2.0 + 30.0;

/// Scene maximum x bounds
pub(crate) const SCENE_MAX_X: f32 = WINDOW_WIDTH as f32 / 2.0 - 25.0;

/// Handles the game scene assets.
pub(crate) struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup the scene when entering main menu.
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
            // Run animation system in all game states.
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(animation_system))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(animation_system))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(animation_system))
            // Cleanup resources on leaving game over state.
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup));
    }
}

/// Scene entities.
#[derive(Resource)]
struct EntityData {
    entities: Vec<Entity>,
}

/// Represents the shop sprite.
#[derive(Component)]
struct Shop;

/// Setup the scene.
fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let mut entities: Vec<Entity> = Vec::new();

    // Setup camera.
    entities.push(commands.spawn(Camera2dBundle::default()).id());

    // Background sprite.
    entities.push(
        commands
            .spawn(SpriteBundle {
                texture: assets.background_image.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0)
                    .with_scale(Vec3::new(BG_SCALE, BG_SCALE, BG_Z)),
                ..default()
            })
            .id(),
    );

    // Animated shop sprite.
    entities.push(
        commands
            .spawn(Shop)
            .insert(SpriteSheetBundle {
                texture_atlas: assets.shop_texture_atlas.clone(),
                transform: Transform {
                    translation: Vec3::new(280.0, -28.5, BG_Z + 0.01),
                    scale: Vec3::new(SHOP_SCALE, SHOP_SCALE, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(AnimationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .id(),
    );

    commands.insert_resource(EntityData { entities });
}

/// Animate the shop sprite.
fn animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Shop>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
