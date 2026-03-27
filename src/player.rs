use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

use crate::{
	CONFIG,
	animation::Animation,
	character::{CharacterBundle, Direction, GroundSensor, Grounded, Velocity},
};

pub struct PlayerConfig {
	pub player_starting_x: f32,
	pub player_starting_y: f32,
	pub player_velocity_x: f32,
	pub player_velocity_y: f32,
	pub spritesheet_cols: u32,
	pub spritesheet_rows: u32,
	pub sprite_path: &'static str,
	pub sprite_tile_width: f32,
	pub sprite_tile_height: f32,
	pub sprite_render_width: f32,
	pub sprite_render_height: f32,
	pub sprite_idx_stand: usize,
	pub sprite_idx_idle: &'static [usize; 4],
	pub sprite_idx_walking: &'static [usize; 9],
	pub sprite_idx_jumping: &'static [usize; 7],
	pub cycle_delay: Duration,
	pub camera_edge_boundary: f32,
}

pub static PLAYER_CONFIG: PlayerConfig = PlayerConfig {
	player_starting_x: CONFIG.window_left_x + 100.0,
	player_starting_y: CONFIG.window_bottom_y + 300.0,
	player_velocity_x: 400.0,
	player_velocity_y: 700.0,
	spritesheet_cols: 8,
	spritesheet_rows: 10,
	sprite_path: "spritesheets/cat_sprite.png",
	sprite_tile_width: 32.,
	sprite_tile_height: 32.,
	sprite_render_width: 64.,
	sprite_render_height: 64.,
	sprite_idx_stand: 0,
	sprite_idx_idle: &[0, 1, 2, 3],
	sprite_idx_walking: &[32, 33, 34, 35, 36, 37, 38, 39, 40],
	sprite_idx_jumping: &[64, 65, 66, 67, 68, 69, 70],
	cycle_delay: Duration::from_millis(70),
	camera_edge_boundary: 100.0,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(
			Update,
			(update_animation, update_direction, update_sprite_direction),
		);
	}
}

#[derive(Component)]
pub struct Player {}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Assets<TextureAtlasLayout>>,
	server: Res<AssetServer>,
) {
	let image_handle: Handle<Image> = server.load(PLAYER_CONFIG.sprite_path);
	let texture_atlas_layout = TextureAtlasLayout::from_grid(
		UVec2::new(
			PLAYER_CONFIG.sprite_tile_width as u32,
			PLAYER_CONFIG.sprite_tile_height as u32,
		),
		PLAYER_CONFIG.spritesheet_cols,
		PLAYER_CONFIG.spritesheet_rows,
		None,
		None,
	);
	let atlas_handle = atlases.add(texture_atlas_layout);

	commands
		.spawn((
			Player {},
			CharacterBundle {
				sprite: Sprite {
					image: image_handle,
					custom_size: Some(Vec2::new(
						PLAYER_CONFIG.sprite_render_width,
						PLAYER_CONFIG.sprite_render_height,
					)),
					texture_atlas: Some(TextureAtlas {
						layout: atlas_handle,
						index: PLAYER_CONFIG.sprite_idx_stand,
					}),
					..default()
				},
				transform: Transform::from_xyz(
					PLAYER_CONFIG.player_starting_x,
					PLAYER_CONFIG.player_starting_y,
					1.0,
				),
				animation: Animation::new(
					PLAYER_CONFIG.sprite_idx_idle,
					PLAYER_CONFIG.cycle_delay,
				),
				velocity: Velocity {
					x: PLAYER_CONFIG.player_velocity_x,
					y: PLAYER_CONFIG.player_velocity_y,
				},
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn((
				Transform::from_xyz(
					0.0,
					-PLAYER_CONFIG.sprite_render_height / 2.0 - 2.0,
					0.0,
				),
				Collider::rectangle(PLAYER_CONFIG.sprite_render_width * 0.8, 4.0),
				Sensor,
				GroundSensor,
				CollidingEntities::default(),
			));
		});
}

fn update_animation(
	mut query: Query<(&LinearVelocity, &Grounded, &mut Animation), With<Player>>,
) {
	let Ok((lin_vel, grounded, mut animation)) = query.single_mut() else {
		return;
	};

	if !grounded.0 {
		// Map vertical velocity to a specific jump frame instead of cycling.
		// Early frames = launch, middle = peak, late = landing.
		let frames = PLAYER_CONFIG.sprite_idx_jumping;
		let i = if lin_vel.y > 400.0 {
			0
		} else if lin_vel.y > 200.0 {
			1
		} else if lin_vel.y > 50.0 {
			2
		} else if lin_vel.y > -50.0 {
			3 // peak
		} else if lin_vel.y > -200.0 {
			4
		} else if lin_vel.y > -400.0 {
			5
		} else {
			6
		};
		// Single-element slice keeps the animate system from cycling.
		animation.sprites = &frames[i..=i];
	} else if lin_vel.x.abs() > 1.0 {
		animation.sprites = PLAYER_CONFIG.sprite_idx_walking;
	} else {
		animation.sprites = PLAYER_CONFIG.sprite_idx_idle;
	}
}

fn update_direction(
	mut commands: Commands,
	query: Query<(Entity, &LinearVelocity), With<Player>>,
) {
	let Ok((player, lin_vel)) = query.single() else {
		return;
	};

	if lin_vel.x > 1.0 {
		commands.entity(player).insert(Direction::Right);
	} else if lin_vel.x < -1.0 {
		commands.entity(player).insert(Direction::Left);
	}
}

fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction)>) {
	let Ok((mut sprite, direction)) = query.single_mut() else {
		return;
	};

	match direction {
		Direction::Right => sprite.flip_x = false,
		Direction::Left => sprite.flip_x = true,
	}
}
