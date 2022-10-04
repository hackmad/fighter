//! Main Menu

use crate::{
    menu_background, menu_border, menu_button, menu_button_interaction_system, menu_button_text,
    menu_root, GameAssets, GameState,
};
use bevy::{app::AppExit, prelude::*};

/// Handles the main menu.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_button_interaction_system)
            .add_system(menu_button_press_system)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup));
    }
}

/// Main menu entities.
struct EntityData {
    entities: Vec<Entity>,
}

/// Represents menu buttons.
#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

/// Setup the main menu.
fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let mut entities: Vec<Entity> = Vec::new();

    entities.push(
        commands
            .spawn_bundle(menu_root())
            .with_children(|parent| {
                // left vertical fill (border)
                parent.spawn_bundle(menu_border()).with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(menu_background())
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(menu_button())
                                .with_children(|parent| {
                                    parent.spawn_bundle(menu_button_text(&assets, "NEW GAME"));
                                })
                                .insert(MenuButton::Play);

                            if cfg!(feature = "native") {
                                parent
                                    .spawn_bundle(menu_button())
                                    .with_children(|parent| {
                                        parent.spawn_bundle(menu_button_text(&assets, "QUIT"));
                                    })
                                    .insert(MenuButton::Quit);
                            }
                        });
                });
            })
            .id(),
    );

    commands.insert_resource(EntityData { entities });
}

/// Processes button press.
fn menu_button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(GameState::InGame)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
