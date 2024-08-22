use crate::animation::Animation;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Component, Copy, Clone)]
pub enum Direction {
	Right,
	Left,
}

pub struct DefaultCharacterConfig {
	starting_x: f32,
	starting_y: f32,
	sprite_render_width: f32,
	sprite_render_height: f32,
	cycle_delay: Duration,
	velocity: Velocity,
	direction: Direction,
}

#[derive(Component, Copy, Clone)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
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
	pub sprite: SpriteBundle,
	pub animation: Animation,
	pub texture_atlas: TextureAtlas,
	pub body: RigidBody,
	pub collider: Collider,
	pub controller: KinematicCharacterController,
	pub velocity: Velocity,
	pub direction: Direction,
}

impl Default for CharacterBundle {
	fn default() -> Self {
		Self {
			sprite: SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(
						DEFAULT_CHARACTER_CONFIG.sprite_render_width,
						DEFAULT_CHARACTER_CONFIG.sprite_render_height,
					)),
					..default()
				},
				transform: Transform {
					translation: Vec3::new(
						DEFAULT_CHARACTER_CONFIG.starting_x,
						DEFAULT_CHARACTER_CONFIG.starting_y,
						1.0,
					),
					scale: Vec3::new(1.0, 1.0, 1.0),
					..default()
				},
				..default()
			},
			texture_atlas: TextureAtlas {
				layout: Handle::default(),
				index: 0,
			},
			body: RigidBody::KinematicPositionBased,
			collider: Collider::cuboid(
				DEFAULT_CHARACTER_CONFIG.sprite_render_width / 2.0,
				DEFAULT_CHARACTER_CONFIG.sprite_render_height / 2.0,
			),
			controller: KinematicCharacterController::default(),
			animation: Animation::new(&[0], DEFAULT_CHARACTER_CONFIG.cycle_delay),
			velocity: DEFAULT_CHARACTER_CONFIG.velocity,
			direction: DEFAULT_CHARACTER_CONFIG.direction,
		}
	}
}
