//! Figher

mod common;
mod countdown_timer;
mod game_over_menu;
mod health;
mod main_menu;
mod menu;
mod player;
mod scene;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_asset_loader::prelude::*;
use common::*;
use countdown_timer::*;
use game_over_menu::*;
use health::*;
use main_menu::*;
use menu::*;
use player::*;
use scene::*;

// Create the app.
pub fn run() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Fighter".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::MainMenu)
                .with_collection::<GameAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ScenePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(CountdownTimerPlugin)
        .add_plugin(GameOverPlugin)
        .run();
}

/// Game assets
#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "fonts/m6x11.ttf")]
    pub(crate) font: Handle<Font>,

    #[asset(path = "images/background_composite.png")]
    pub(crate) background_image: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 118.0, tile_size_y = 128.0, columns = 6, rows = 1))]
    #[asset(path = "images/shop_anim.png")]
    pub(crate) shop_texture_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 200.0, tile_size_y = 200.0, columns = 8, rows = 9))]
    #[asset(path = "images/player_one.png")]
    pub(crate) player_one_texture_atlas: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 200.0, tile_size_y = 200.0, columns = 8, rows = 8))]
    #[asset(path = "images/player_two.png")]
    pub(crate) player_two_texture_atlas: Handle<TextureAtlas>,
}

/// Game states.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GameState {
    AssetLoading,
    MainMenu,
    InGame,
    GameOver,
}
