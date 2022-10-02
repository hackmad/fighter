//! Common

use bevy::prelude::*;

/// Gravity strength.
pub const GRAVITY: f32 = 0.7;

/// Window width.
pub const WINDOW_WIDTH: f32 = 1024.0;

/// Window height.
pub const WINDOW_HEIGHT: f32 = 576.0;

/* Define z-coordinates for images/sprites so we can control draw order */
pub const BG_Z: f32 = 0.0;
pub const PLAYER_Z: f32 = 0.2;
pub const HEALTH_BAR_Z: f32 = 0.4;
pub const COUNTDOWN_TIMER_Z: f32 = 0.6;

/// Timer for animating sprites.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
