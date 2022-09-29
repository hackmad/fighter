//! Common

use bevy::prelude::*;

/// Gravity strength.
pub const GRAVITY: f32 = 0.7;

/// Scaling factor for background sprite.
pub const BG_SCALE: f32 = 3.2;

/// Ground location along y-axis.
pub const GROUND_Y: f32 = -66.0 * BG_SCALE;

/// Scaling factor for shop sprite.
pub const SHOP_SCALE: f32 = 2.85;

/// Scaling factor for player sprite.
pub const PLAYER_SCALE: f32 = 2.75;

/// Initial velocity for player jumps.
pub const JUMP_VELOCITY: f32 = 20.0;

/// Velocity for horizontal player movement.
pub const HORIZ_VELOCITY: f32 = 5.0;

/// Window width.
pub const WINDOW_WIDTH: f32 = 1024.0;

/// Window height.
pub const WINDOW_HEIGHT: f32 = 576.0;

/// Collider alpha (used for displaying collider for debugging).
pub const COLLIDER_ALPHA: f32 = 0.5;

/// Timer for animating sprites.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
