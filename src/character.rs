use crate::animation::Animation;
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Copy, Clone)]
pub enum Direction {
	Right,
	Left,
}

#[derive(Component, Copy, Clone)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct GroundSensor;

pub struct DefaultCharacterConfig {
	starting_x: f32,
	starting_y: f32,
	sprite_render_width: f32,
	sprite_render_height: f32,
	cycle_delay: Duration,
	velocity: Velocity,
	direction: Direction,
}

pub static DEFAULT_CHARACTER_CONFIG: DefaultCharacterConfig =
	DefaultCharacterConfig {
		starting_x: 0.0,
		starting_y: 0.0,
		sprite_render_width: 64.0,
		sprite_render_height: 64.0,
		cycle_delay: Duration::from_millis(100),
		velocity: Velocity { x: 300.0, y: 650.0 },
		direction: Direction::Right,
	};

#[derive(Bundle)]
pub struct CharacterBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub animation: Animation,
	pub body: RigidBody,
	pub collider: Collider,
	pub locked_axes: LockedAxes,
	pub linear_velocity: LinearVelocity,
	pub velocity: Velocity,
	pub direction: Direction,
	pub grounded: Grounded,
}

impl Default for CharacterBundle {
	fn default() -> Self {
		Self {
			sprite: Sprite {
				custom_size: Some(Vec2::new(
					DEFAULT_CHARACTER_CONFIG.sprite_render_width,
					DEFAULT_CHARACTER_CONFIG.sprite_render_height,
				)),
				..default()
			},
			transform: Transform::from_xyz(
				DEFAULT_CHARACTER_CONFIG.starting_x,
				DEFAULT_CHARACTER_CONFIG.starting_y,
				1.0,
			),
			body: RigidBody::Dynamic,
			collider: Collider::rectangle(
				DEFAULT_CHARACTER_CONFIG.sprite_render_width / 2.5,
				DEFAULT_CHARACTER_CONFIG.sprite_render_height,
			),
			locked_axes: LockedAxes::ROTATION_LOCKED,
			linear_velocity: LinearVelocity::ZERO,
			animation: Animation::new(&[0], DEFAULT_CHARACTER_CONFIG.cycle_delay),
			velocity: DEFAULT_CHARACTER_CONFIG.velocity,
			direction: DEFAULT_CHARACTER_CONFIG.direction,
			grounded: Grounded(false),
		}
	}
}
