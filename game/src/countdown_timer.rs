//! Countdown Timer

use crate::{common::*, GameAssets, GameState};
use bevy::prelude::*;

/// Starting value for countdown timer.
const COUNTDOWN_TIMER_START: u16 = 30;

/// Handles the countdown timer.
pub struct CountdownTimerPlugin;

impl Plugin for CountdownTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CountdownCompleteEvent>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(countdown_system))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

/// Countdown timer entities.
#[derive(Resource)]
struct EntityData {
    entities: Vec<Entity>,
}

/// Represents the countdown timer.
#[derive(Component)]
struct CountdownTimer {
    remaining: u16,
    done: bool,
}

impl Default for CountdownTimer {
    fn default() -> Self {
        Self {
            remaining: COUNTDOWN_TIMER_START,
            done: false,
        }
    }
}

/// Used to communicate end of countdown.
pub struct CountdownCompleteEvent;

/// Setup the countdown timer.
fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    let timer_pos = Vec3::new(0.0, 225.0, COUNTDOWN_TIMER_Z);
    let timer_size = Vec3::new(95.0, 40.0, 1.0);

    let mut entities: Vec<Entity> = Vec::new();

    // Background of countdown timer.
    entities.push(
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform {
                    translation: timer_pos,
                    scale: timer_size + Vec3::new(4.0, 4.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    entities.push(
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    ..default()
                },
                transform: Transform {
                    translation: timer_pos + Vec3::new(0.0, 0.0, 0.01),
                    scale: timer_size,
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    // The timer.
    entities.push(
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                    position: UiRect {
                        top: Val::Px(35.0),
                        left: Val::Px(0.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            })
            .with_children(|timer| {
                timer
                    .spawn(TextBundle {
                        text: Text::from_section(
                            format!("{}", COUNTDOWN_TIMER_START),
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        style: Style {
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(AnimationTimer(Timer::from_seconds(
                        1.0,
                        TimerMode::Repeating,
                    )))
                    .insert(CountdownTimer::default());
            })
            .id(),
    );

    commands.insert_resource(EntityData { entities });
}

/// Update the timer.
fn countdown_system(
    time: Res<Time>,
    mut countdown_timer_query: Query<&mut CountdownTimer>,
    mut text_query: Query<(&mut Text, &mut AnimationTimer), With<CountdownTimer>>,
    mut countdown_complete_events: EventWriter<CountdownCompleteEvent>,
) {
    let (mut text, mut animation_timer) = text_query.single_mut();
    let mut countdown_timer = countdown_timer_query.single_mut();

    animation_timer.tick(time.delta());
    if animation_timer.just_finished() {
        if !countdown_timer.done {
            if countdown_timer.remaining > 0 {
                countdown_timer.remaining -= 1;
                text.sections[0].value = format!("{}", countdown_timer.remaining);
            } else {
                countdown_complete_events.send(CountdownCompleteEvent);
                countdown_timer.done = true;
            }
        }
    }
}

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
