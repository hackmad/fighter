//! Main Menu

use crate::{GameAssets, GameState};
use bevy::{app::AppExit, prelude::*};

/// Handles the main menu.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system)
            .add_system(button_press_system)
            .add_system_set(
                // Setup main menu.
                SystemSet::on_enter(GameState::MainMenu).with_system(setup),
            )
            .add_system_set(
                // Cleanup resources.
                SystemSet::on_exit(GameState::MainMenu).with_system(cleanup),
            );
    }
}

/// Main menu data.
struct MainMenuData {
    ui_root: Entity,
}

/// Menu buttons.
#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

/// Setup the main menu.
fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let ui_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn_bundle(border()).with_children(|parent| {
                // left vertical fill (content)
                parent
                    .spawn_bundle(menu_background())
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(button())
                            .with_children(|parent| {
                                parent.spawn_bundle(button_text(&assets, "NEW GAME"));
                            })
                            .insert(MenuButton::Play);

                        parent
                            .spawn_bundle(button())
                            .with_children(|parent| {
                                parent.spawn_bundle(button_text(&assets, "QUIT"));
                            })
                            .insert(MenuButton::Quit);
                    });
            });
        })
        .id();

    commands.insert_resource(MainMenuData { ui_root });
}

/// Cleanup menu resources.
fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
}

/// Handle button interactions.
fn button_system(
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = UiColor(Color::rgba(0.35, 0.75, 0.35, 0.8));
            }
            Interaction::Hovered => {
                *color = UiColor(Color::rgba(0.25, 0.25, 0.25, 0.8));
            }
            Interaction::None => {
                *color = UiColor(Color::rgba(0.15, 0.15, 0.15, 0.8));
            }
        }
    }
}

/// Processes pressed button.
fn button_press_system(
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

/// Create the root of the main menu layout.
fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: UiColor(Color::NONE),
        ..default()
    }
}

/// Add a border to the menu.
fn border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        color: UiColor(Color::rgba(0.25, 0.25, 0.25, 0.5)),
        ..default()
    }
}

/// Setup menu background.
fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        color: UiColor(Color::rgba(1.0, 1.0, 1.0, 0.5)),
        ..default()
    }
}

fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: UiColor(Color::rgba(0.15, 0.15, 0.15, 0.8)),
        ..default()
    }
}

fn button_text(assets: &Res<GameAssets>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: assets.font.clone(),
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ..default()
    };
}
