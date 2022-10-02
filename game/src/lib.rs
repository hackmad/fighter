//! Figher

mod common;
mod countdown_timer;
mod health;
mod player;
mod scene;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
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
        .add_plugins(DefaultPlugins)
        .add_plugin(ScenePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(CountdownTimerPlugin)
        .add_startup_system(setup)
        .add_system(game_over_system)
        //.add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct GameOver;

/// Setup the game.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // NOTE: The NodeBundle/TextBundle for the timer makes it super hard to use a similar
    // setup here. So we resort to positioning text manually and using a fixed length string
    // with padded spaces around it to get it close enough.
    let font: Handle<Font> = asset_server.load("m6x11.ttf");
    commands
        .spawn()
        .insert_bundle(TextBundle {
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font,
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
