//! Health

use crate::common::*;
use crate::HealthUpdateEvent;
use crate::Player;
use bevy::prelude::*;
use std::collections::HashMap;

/// Health bar maximum width for 100% health.
const HEALTH_BAR_MAX_WIDTH: f32 = 400.0;

/// Handles the health.
pub(crate) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(health_update_system);
    }
}

/// Represents the health bar of a player storing percent health.
#[derive(Component)]
struct HealthBar(u8);
impl Default for HealthBar {
    fn default() -> Self {
        Self(100)
    }
}

/// Setup.
fn setup(mut commands: Commands) {
    spawn_health_bar(&mut commands, Player::One);
    spawn_health_bar(&mut commands, Player::Two);
}

fn spawn_health_bar(commands: &mut Commands, player: Player) {
    let health_bar_pos = match player {
        Player::One => Vec3::new(-250.0, 225.0, HEALTH_BAR_Z),
        Player::Two => Vec3::new(250.0, 225.0, HEALTH_BAR_Z),
    };

    let health_bar_size = Vec3::new(HEALTH_BAR_MAX_WIDTH, 30.0, 1.0);

    // Background of health bar.
    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            translation: health_bar_pos,
            scale: health_bar_size + Vec3::new(4.0, 4.0, 0.0),
            ..default()
        },
        ..default()
    });

    // Damage reveal for health bar.
    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: health_bar_pos + Vec3::new(0.0, 0.0, 0.01),
            scale: health_bar_size,
            ..default()
        },
        ..default()
    });

    // Health bar.
    commands
        .spawn()
        .insert(HealthBar::default())
        .insert(player)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.502, 0.549, 0.984),
                ..default()
            },
            transform: Transform {
                translation: health_bar_pos + Vec3::new(0.0, 0.0, 0.02),
                scale: health_bar_size,
                ..default()
            },
            ..default()
        });
}

/// Process player health updates.
fn health_update_system(
    mut health_update_events: EventReader<HealthUpdateEvent>,
    mut health_bar_query: Query<(&Player, &mut HealthBar, &mut Transform), With<Sprite>>,
) {
    let mut current_health: HashMap<Player, u8> = HashMap::new();

    if !health_update_events.is_empty() {
        for event in health_update_events.iter() {
            if let Some(health) = current_health.get_mut(&event.player) {
                *health = event.health;
            } else {
                current_health.insert(event.player, event.health);
            }
        }
    }

    for (player, mut health_bar, mut transform) in &mut health_bar_query {
        if let Some(health) = current_health.get(player) {
            let diff = (health_bar.0 - health) as f32;
            let hp = diff / 100.0;
            match player {
                Player::One => transform.translation.x += hp * HEALTH_BAR_MAX_WIDTH / 2.0,
                Player::Two => transform.translation.x -= hp * HEALTH_BAR_MAX_WIDTH / 2.0,
            }
            transform.scale.x -= hp * HEALTH_BAR_MAX_WIDTH;
            health_bar.0 = *health;
        }
    }
}
