//! Common

use bevy::prelude::*;

/// Gravity strength.
pub const GRAVITY: f32 = 0.7;

/// Window width.
pub const WINDOW_WIDTH: f32 = 1024.0;

/// Window height.
pub const WINDOW_HEIGHT: f32 = 576.0;

/// Collider alpha (used for displaying collider for debugging).
pub const COLLIDER_ALPHA: f32 = 0.5;

/// Timer for animating sprites.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
