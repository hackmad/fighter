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
            .add_system_set(
                // Setup countdown timer.
                SystemSet::on_enter(GameState::InGame).with_system(setup),
            )
            .add_system_set(
                // Enable countdown timer during game play.
                SystemSet::on_update(GameState::InGame).with_system(countdown_system),
            );
    }
}

/// The countdown timer.
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

    // Background of countdown timer.
    commands.spawn().insert_bundle(SpriteBundle {
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
    });
    commands.spawn().insert_bundle(SpriteBundle {
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
    });

    // The timer.
    commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position: UiRect {
                    top: Val::Px(-482.5),
                    left: Val::Px(0.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|timer| {
            timer
                .spawn_bundle(TextBundle {
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
                .insert(AnimationTimer(Timer::from_seconds(1.0, true)))
                .insert(CountdownTimer::default());
        });
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
