//! Figher

mod common;
mod countdown_timer;
mod health;
mod player;
mod scene;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_asset_loader::prelude::*;
use common::*;
use countdown_timer::*;
use health::*;
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
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .with_collection::<GameAssets>(),
        )
        .add_state(GameStates::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(ScenePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(CountdownTimerPlugin)
        .add_system_set(SystemSet::on_enter(GameStates::Next).with_system(setup))
        .add_system_set(SystemSet::on_update(GameStates::Next).with_system(game_over_system))
        //.add_system(bevy::window::close_on_esc)
        .run();
}

/// Game assets
#[derive(AssetCollection)]
pub(crate) struct GameAssets {
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
enum GameStates {
    AssetLoading,
    Next,
}

/// Game over message.
#[derive(Component)]
struct GameOver;

/// Setup the game.
fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // NOTE: The NodeBundle/TextBundle for the timer makes it super hard to use a similar
    // setup here. So we resort to positioning text manually and using a fixed length string
    // with padded spaces around it to get it close enough.
    commands
        .spawn()
        .insert_bundle(TextBundle {
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(435.0),
                    top: Val::Px(250.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(GameOver);
}

/// Checks if game is over.
fn game_over_system(
    mut countdown_complete_events: EventReader<CountdownCompleteEvent>,
    mut text_query: Query<(&mut Text, &mut Visibility), With<GameOver>>,
    health_query: Query<(&Player, &Health)>,
    mut health_update_events: EventReader<HealthUpdateEvent>,
) {
    let mut game_over = false;

    // Check if countdown is complete.
    if !countdown_complete_events.is_empty() {
        for _event in countdown_complete_events.iter() {
            game_over = true;
            break;
        }
    }

    if !game_over {
        // Check if one player has 0 health.
        if !health_update_events.is_empty() {
            for event in health_update_events.iter() {
                if event.health == 0 {
                    game_over = true;
                    break;
                }
            }
        }
    }

    if game_over {
        // Retrieve health of both players to determine weather there is a clear winner or a draw.
        let mut healths = [0_u8; 2];
        for (player, health) in health_query.iter() {
            healths[player.index()] = health.0;
        }

        let (mut text, mut visibility) = text_query.single_mut();

        if healths[0] > healths[1] {
            text.sections[0].value = "PLAYER 1 WINS".to_string();
        } else if healths[1] > healths[0] {
            text.sections[0].value = "PLAYER 2 WINS".to_string();
        } else {
            text.sections[0].value = "     DRAW    ".to_string();
        }

        visibility.is_visible = true;
    }
}
