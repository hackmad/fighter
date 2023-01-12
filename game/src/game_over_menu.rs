//! Gamr Over Menu

use crate::{
    menu_background, menu_border, menu_button, menu_button_interaction_system, menu_button_text,
    menu_root, GameAssets, GameState, Health, Player,
};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

/// Message backround color.
const MESSAGE_BACKGROUND_COLOR: Color = Color::rgba(0.05, 0.05, 0.05, 0.9);

/// Message text color.
const MESSAGE_TEXT_COLOR: Color = Color::WHITE;

/// Handles the game over display.
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu_button_interaction_system)
            .add_system(menu_button_press_system)
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(input_system))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup));
    }
}

/// Countdown timer entities.
#[derive(Resource)]
struct EntityData {
    entities: Vec<Entity>,
}

/// Represents menu buttons.
#[derive(Component)]
enum MenuButton {
    Continue,
}

/// Setup the players.
fn setup(mut commands: Commands, assets: Res<GameAssets>, health_query: Query<(&Player, &Health)>) {
    let mut entities: Vec<Entity> = Vec::new();

    // Retrieve health of both players to determine weather there is a clear winner or a draw.
    let mut healths = [0_u8; 2];
    for (player, health) in health_query.iter() {
        healths[player.index()] = health.0;
    }

    let msg = if healths[0] > healths[1] {
        "PLAYER 1 WINS"
    } else if healths[1] > healths[0] {
        "PLAYER 2 WINS"
    } else {
        "DRAW"
    };

    entities.push(
        commands
            .spawn(menu_root())
            .with_children(|parent| {
                // left vertical fill (border)
                parent.spawn(menu_border()).with_children(|parent| {
                    // left vertical fill (content)
                    parent.spawn(menu_background()).with_children(|parent| {
                        parent
                            .spawn(menu_button())
                            .with_children(|parent| {
                                parent.spawn(menu_button_text(&assets, "CONTINUE"));
                                parent.spawn(ImageBundle {
                                    image: UiImage(assets.return_key_image.clone()),
                                    transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
                                    ..default()
                                });
                            })
                            .insert(MenuButton::Continue);

                        parent.spawn(message()).with_children(|parent| {
                            parent.spawn(message_text(&assets, msg));
                        });
                    });
                });
            })
            .id(),
    );

    commands.insert_resource(EntityData { entities });
}

/// Create a message.
fn message() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(MESSAGE_BACKGROUND_COLOR),
        ..default()
    }
}

/// Create message text.
fn message_text(assets: &Res<GameAssets>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: assets.font.clone(),
                font_size: 24.0,
                color: MESSAGE_TEXT_COLOR,
                ..default()
            },
        ),
        ..default()
    };
}

/// Processes button press.
fn menu_button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Continue => state
                    .set(GameState::MainMenu)
                    .expect("Couldn't switch state to MainMenu"),
            };
        }
    }
}

/// Handle keyboard input.
fn input_system(mut keyboard_input: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state
            .set(GameState::MainMenu)
            .expect("Couldn't switch state to MainMenu");
        keyboard_input.clear_just_pressed(KeyCode::Return);
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>, audio: Res<Audio>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
    audio.stop();
}
