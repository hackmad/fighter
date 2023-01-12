//! Health

use crate::{GameState, HealthUpdateEvent, Player, HEALTH_BAR_Z};
use bevy::prelude::*;
use std::collections::HashMap;

/// Health bar maximum width for 100% health.
const HEALTH_BAR_MAX_WIDTH: f32 = 400.0;

/// Health bar size.
const HEALTH_BAR_SIZE: Vec3 = Vec3::new(HEALTH_BAR_MAX_WIDTH, 30.0, 1.0);

/// Health bar positions.
const HEALTH_BAR_POS: [Vec3; 2] = [
    Vec3::new(-250.0, 225.0, HEALTH_BAR_Z),
    Vec3::new(250.0, 225.0, HEALTH_BAR_Z),
];

/// Handles the health.
pub(crate) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(update_system))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

/// Health bar entities.
#[derive(Resource)]
struct EntityData {
    entities: Vec<Entity>,
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
    let mut entities: Vec<Entity> = Vec::new();

    for player in [Player::One, Player::Two] {
        // Background of health bar.
        entities.push(
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    transform: Transform {
                        translation: HEALTH_BAR_POS[player.index()],
                        scale: HEALTH_BAR_SIZE + Vec3::new(4.0, 4.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .id(),
        );

        // Damage reveal for health bar.
        entities.push(
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform {
                        translation: HEALTH_BAR_POS[player.index()] + Vec3::new(0.0, 0.0, 0.01),
                        scale: HEALTH_BAR_SIZE,
                        ..default()
                    },
                    ..default()
                })
                .id(),
        );

        // Health bar.
        entities.push(
            commands
                .spawn(HealthBar::default())
                .insert(player)
                .insert(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.502, 0.549, 0.984),
                        ..default()
                    },
                    transform: Transform {
                        translation: HEALTH_BAR_POS[player.index()] + Vec3::new(0.0, 0.0, 0.02),
                        scale: HEALTH_BAR_SIZE,
                        ..default()
                    },
                    ..default()
                })
                .id(),
        );
    }

    commands.insert_resource(EntityData { entities });
}

/// Process player health updates.
fn update_system(
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

/// Cleanup resources.
fn cleanup(mut commands: Commands, entity_data: Res<EntityData>) {
    for entity in entity_data.entities.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}
