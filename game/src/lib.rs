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

/// Setup the game.
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

/// Checks if game is over.
fn game_over_system(
    mut countdown_complete_events: EventReader<CountdownCompleteEvent>,
    mut health_update_events: EventReader<HealthUpdateEvent>,
) {
    let mut game_over = false;
    if !countdown_complete_events.is_empty() {
        for _event in countdown_complete_events.iter() {
            game_over = true;
            break;
        }
    }

    if !game_over {
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
        println!("GAME OVER");
    }
}
