//! Menu

use crate::GameAssets;
use bevy::prelude::*;

/// Menu background color.
pub(crate) const MENU_BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);

/// Menu border color.
pub(crate) const MENU_BORDER_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);

/// Menu button clicked color.
pub(crate) const MENU_BUTTON_CLICKED_COLOR: Color = Color::rgba(0.35, 0.75, 0.35, 0.8);

/// Menu button hover color.
pub(crate) const MENU_BUTTON_HOVER_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.8);

/// Menu button color.
pub(crate) const MENU_BUTTON_COLOR: Color = Color::rgba(0.05, 0.05, 0.05, 0.9);

/// Menu button text color.
pub(crate) const MENU_BUTTON_TEXT_COLOR: Color = Color::WHITE;

/// Handle button interactions.
pub(crate) fn menu_button_interaction_system(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(MENU_BUTTON_CLICKED_COLOR);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(MENU_BUTTON_HOVER_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(MENU_BUTTON_COLOR);
            }
        }
    }
}

/// Create the root of the main menu layout.
pub(crate) fn menu_root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    }
}

/// Add a border to the menu.
pub(crate) fn menu_border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        background_color: BackgroundColor(MENU_BORDER_COLOR),
        ..default()
    }
}

/// Setup menu background.
pub(crate) fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: BackgroundColor(MENU_BACKGROUND_COLOR),
        ..default()
    }
}

/// Create a button.
pub(crate) fn menu_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(MENU_BUTTON_COLOR),
        ..default()
    }
}

/// Create button text.
pub(crate) fn menu_button_text(assets: &Res<GameAssets>, label: &str) -> TextBundle {
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
                color: MENU_BUTTON_TEXT_COLOR,
                ..default()
            },
        ),
        ..default()
    };
}
