//! Scene

use crate::common::*;
use bevy::app::Plugin;
use bevy::prelude::*;

/// Handles the game scene assets.
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(animation_system);
    }
}

/// Represents the shop sprite.
#[derive(Component)]
struct Shop;

/// Setup the scene.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
    let shop_atlas_handle = load_sprite(
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
}

/// Load an animated sprite sheet for the shop.
fn load_sprite(
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
