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
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(input_system))
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
                                    parent.spawn_bundle(ImageBundle {
                                        image: UiImage(assets.return_key_image.clone()),
                                        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
                                        ..default()
                                    });
                                })
                                .insert(MenuButton::Play);

                            if cfg!(feature = "native") {
                                // In browser this does stop the game but it shows as frozen. So
                                // best not to add it. User can just close the window/tab.
                                parent
                                    .spawn_bundle(menu_button())
                                    .with_children(|parent| {
                                        parent.spawn_bundle(menu_button_text(&assets, "QUIT"));
                                        parent.spawn_bundle(ImageBundle {
                                            image: UiImage(assets.escape_key_image.clone()),
                                            transform: Transform::from_scale(Vec3::new(
                                                0.58, 0.58, 0.58,
                                            )),
                                            ..default()
                                        });
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

/// Handle keyboard input.
fn input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state
            .set(GameState::InGame)
            .expect("Couldn't switch state to InGame");
        keyboard_input.clear_just_pressed(KeyCode::Return);
    } else if cfg!(feature = "native") && keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
        keyboard_input.clear_just_pressed(KeyCode::Escape);
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
