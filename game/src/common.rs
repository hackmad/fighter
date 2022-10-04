//! Common

use bevy::prelude::*;

/// Window width.
pub(crate) const WINDOW_WIDTH: f32 = 1024.0;

/// Window height.
pub(crate) const WINDOW_HEIGHT: f32 = 576.0;

/* Define z-coordinates for images/sprites so we can control draw order */
pub(crate) const BG_Z: f32 = 0.0;
pub(crate) const PLAYER_Z: f32 = 0.2;
pub(crate) const HEALTH_BAR_Z: f32 = 0.4;
pub(crate) const COUNTDOWN_TIMER_Z: f32 = 0.6;

/// Timer for animating sprites.
#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(pub(crate) Timer);
