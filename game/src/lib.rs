mod common;
mod player;
mod scene;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use common::*;
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
        .add_plugin(PlayerPlugin)
        .add_plugin(ScenePlugin)
        .add_startup_system(setup)
        //.add_system(bevy::window::close_on_esc)
        .run();
}

/// Setup the game.
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
