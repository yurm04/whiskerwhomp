use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
	animation::Animation,
	character::{CharacterBundle, Direction, Velocity},
	CONFIG,
};

pub struct PlayerConfig {
	pub player_starting_x: f32,
	pub player_starting_y: f32,
	pub player_velocity_x: f32,
	pub player_velocity_y: f32,
	pub max_jump_height: f32,
	pub jump_duration: f32,
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
	player_velocity_y: 850.0,
	max_jump_height: 4500.0,
	jump_duration: 0.20,
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
		app
			.add_systems(Startup, setup)
			.add_systems(Update, apply_movement_animation)
			.add_systems(Update, apply_idle_animation)
			.add_systems(Update, apply_jumping_animation)
			.add_systems(Update, update_direction)
			.add_systems(Update, update_sprite_direction);
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
	let texture_atlas = TextureAtlasLayout::from_grid(
		UVec2::new(
			PLAYER_CONFIG.sprite_tile_width as u32,
			PLAYER_CONFIG.sprite_tile_height as u32,
		),
		PLAYER_CONFIG.spritesheet_cols,
		PLAYER_CONFIG.spritesheet_rows,
		None,
		None,
	);
	let atlas_handle = atlases.add(texture_atlas);

	commands.spawn((
		Player {},
		CharacterBundle {
			sprite: SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(
						PLAYER_CONFIG.sprite_render_width,
						PLAYER_CONFIG.sprite_render_height,
					)),
					..Default::default()
				},
				texture: image_handle,
				transform: Transform {
					translation: Vec3::new(
						PLAYER_CONFIG.player_starting_x,
						PLAYER_CONFIG.player_starting_y,
						1.0,
					),
					scale: Vec3::new(1.0, 1.0, 1.0),
					..default()
				},
				..default()
			},
			texture_atlas: TextureAtlas {
				layout: atlas_handle,
				index: PLAYER_CONFIG.sprite_idx_stand,
			},
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
	));
}

fn apply_movement_animation(
	mut query: Query<(&KinematicCharacterControllerOutput, &mut Animation)>,
) {
	if query.is_empty() {
		return;
	}

	let (output, mut animation) = query.single_mut();
	if output.desired_translation.x != 0.0 && output.grounded {
		animation.sprites = PLAYER_CONFIG.sprite_idx_walking;
	}
}

fn apply_idle_animation(
	mut query: Query<(&KinematicCharacterControllerOutput, &mut Animation)>,
) {
	if query.is_empty() {
		return;
	}

	let (output, mut animation) = query.single_mut();
	if output.desired_translation.x == 0.0 && output.grounded {
		animation.sprites = PLAYER_CONFIG.sprite_idx_idle;
	}
}

fn apply_jumping_animation(
	mut query: Query<(&KinematicCharacterControllerOutput, &mut Animation)>,
) {
	if query.is_empty() {
		return;
	}

	let (output, mut animation) = query.single_mut();
	if !output.grounded {
		animation.sprites = PLAYER_CONFIG.sprite_idx_jumping;
	}
}

fn update_direction(
	mut commands: Commands,
	query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();

	if output.desired_translation.x > 0.0 {
		commands.entity(player).insert(Direction::Right);
	} else if output.desired_translation.x < 0.0 {
		commands.entity(player).insert(Direction::Left);
	}
}

fn update_sprite_direction(mut query: Query<(&mut Sprite, &Direction)>) {
	if query.is_empty() {
		return;
	}

	let (mut sprite, direction) = query.single_mut();

	match direction {
		Direction::Right => sprite.flip_x = false,
		Direction::Left => sprite.flip_x = true,
	}
}
