use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{animation::Animation, CONFIG};

struct PlayerConfig {
	player_starting_x: f32,
	player_starting_y: f32,
	player_velocity_x: f32,
	player_velocity_y: f32,
	max_jump_height: f32,
	jump_duration: f32,
	spritesheet_cols: u32,
	spritesheet_rows: u32,
	sprite_path: &'static str,
	sprite_tile_width: f32,
	sprite_tile_height: f32,
	sprite_render_width: f32,
	sprite_render_height: f32,
	sprite_idx_stand: usize,
	sprite_idx_walking: &'static [usize; 9],
	sprite_idx_jump: usize,
	cycle_delay: Duration,
}

static PLAYER_CONFIG: PlayerConfig = PlayerConfig {
	player_starting_x: CONFIG.window_left_x + 100.0,
	player_starting_y: CONFIG.window_bottom_y + 300.0,
	player_velocity_x: 400.0,
	player_velocity_y: 850.0,
	max_jump_height: 230.0,
	jump_duration: 0.25,
	spritesheet_cols: 8,
	spritesheet_rows: 10,
	sprite_path: "spritesheets/cat_sprite.png",
	sprite_tile_width: 32.,
	sprite_tile_height: 32.,
	sprite_render_width: 64.,
	sprite_render_height: 64.,
	sprite_idx_stand: 0,
	sprite_idx_walking: &[32, 33, 34, 35, 36, 37, 38, 39, 40],
	sprite_idx_jump: 66,
	cycle_delay: Duration::from_millis(70),
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup)
			.add_systems(Update, movement)
			.add_systems(Update, jump)
			.add_systems(Update, rise)
			.add_systems(Update, fall)
			.add_systems(Update, apply_movement_animation)
			.add_systems(Update, apply_idle_sprite)
			.add_systems(Update, apply_jump_sprite)
			.add_systems(Update, update_direction)
			.add_systems(Update, update_sprite_direction)
			.add_systems(Update, camera_follow_system);
	}
}

#[derive(Component)]
enum Direction {
	Right,
	Left,
}

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

	commands
		.spawn((
			SpriteBundle {
				sprite: Sprite::default(),

				texture: image_handle,
				transform: Transform {
					translation: Vec3::new(
						PLAYER_CONFIG.player_starting_x,
						PLAYER_CONFIG.player_starting_y,
						0.0,
					),
					scale: Vec3::new(
						PLAYER_CONFIG.sprite_render_width / PLAYER_CONFIG.sprite_tile_width,
						PLAYER_CONFIG.sprite_render_height
							/ PLAYER_CONFIG.sprite_tile_height,
						1.0,
					),
					..default()
				},
				..default()
			},
			TextureAtlas {
				layout: atlas_handle,
				index: PLAYER_CONFIG.sprite_idx_stand,
			},
		))
		.insert(RigidBody::KinematicPositionBased)
		.insert(Collider::cuboid(
			PLAYER_CONFIG.sprite_tile_width / 2.0,
			PLAYER_CONFIG.sprite_tile_height / 2.0,
		))
		.insert(KinematicCharacterController::default())
		.insert(Direction::Right);
}

fn movement(
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut query: Query<&mut KinematicCharacterController>,
) {
	let mut player = query.single_mut();

	let mut movement = 0.0;

	if input.pressed(KeyCode::ArrowRight) {
		movement += time.delta_seconds() * PLAYER_CONFIG.player_velocity_x;
	}

	if input.pressed(KeyCode::ArrowLeft) {
		movement += time.delta_seconds() * PLAYER_CONFIG.player_velocity_x * -1.0;
	}

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
		None => player.translation = Some(Vec2::new(movement, 0.0)),
	}
}

#[derive(Component)]
struct Jump(f32);

// Define control points for the cubic Bezier curve
const CONTROL_POINTS: [(f32, f32); 4] = [
	(0.0, 0.0),      // Starting point
	(0.0075, 0.009), // Control point 1
	(0.0075, 0.009), // Control point 2
	(0.01, 0.0),     // End point
];

// Cubic Bezier function
fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
	let u = 1.0 - t;
	let tt = t * t;
	let uu = u * u;
	let uuu = uu * u;
	let ttt = tt * t;

	uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
}

#[allow(clippy::type_complexity)]
fn jump(
	input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	query: Query<
		(Entity, &KinematicCharacterControllerOutput),
		(With<KinematicCharacterController>, Without<Jump>),
	>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();

	if input.pressed(KeyCode::ArrowUp) && output.grounded {
		commands.entity(player).insert(Jump(0.0));
	}
}

fn rise(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
	if query.is_empty() {
		return;
	}

	let (entity, mut player, mut jump) = query.single_mut();

	let jump_duration = PLAYER_CONFIG.jump_duration;
	let jump_height = PLAYER_CONFIG.max_jump_height;

	// Calculate the time passed as a fraction of the total jump duration
	let t = jump.0 / jump_duration;
	if t >= 1.0 {
		commands.entity(entity).remove::<Jump>();
		return;
	}

	// Calculate the new height using the cubic Bezier curve
	let new_height = cubic_bezier(
		t,
		CONTROL_POINTS[0].1,
		CONTROL_POINTS[1].1,
		CONTROL_POINTS[2].1,
		CONTROL_POINTS[3].1,
	) * jump_height;

	// Update the player's vertical position
	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, new_height * 8.0)),
		None => player.translation = Some(Vec2::new(0.0, new_height * 8.0)),
	}

	// Update the jump timer
	jump.0 += time.delta().as_secs_f32();
}

fn fall(
	time: Res<Time>,
	mut query: Query<&mut KinematicCharacterController, Without<Jump>>,
) {
	if query.is_empty() {
		return;
	}

	let mut player = query.single_mut();
	let movement =
		time.delta().as_secs_f32() * (PLAYER_CONFIG.player_velocity_y / 1.5) * -1.0;

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
		None => player.translation = Some(Vec2::new(0.0, movement)),
	}
}

fn apply_movement_animation(
	mut commands: Commands,
	query: Query<
		(Entity, &KinematicCharacterControllerOutput),
		Without<Animation>,
	>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();
	if output.desired_translation.x != 0.0 && output.grounded {
		commands.entity(player).insert(Animation::new(
			PLAYER_CONFIG.sprite_idx_walking,
			PLAYER_CONFIG.cycle_delay,
		));
	}
}

fn apply_idle_sprite(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut TextureAtlas,
	)>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output, mut sprite) = query.single_mut();
	if output.desired_translation.x == 0.0 && output.grounded {
		commands.entity(player).remove::<Animation>();
		sprite.index = PLAYER_CONFIG.sprite_idx_stand
	}
}

fn apply_jump_sprite(
	mut commands: Commands,
	mut query: Query<(
		Entity,
		&KinematicCharacterControllerOutput,
		&mut TextureAtlas,
	)>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output, mut sprite) = query.single_mut();
	if !output.grounded {
		commands.entity(player).remove::<Animation>();
		sprite.index = PLAYER_CONFIG.sprite_idx_jump
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

fn camera_follow_system(
	player_query: Query<&Transform, With<KinematicCharacterControllerOutput>>,
	mut camera_query: Query<
		&mut Transform,
		(With<Camera>, Without<KinematicCharacterControllerOutput>),
	>,
) {
	if let Ok(player_transform) = player_query.get_single() {
		if let Ok(mut camera_transform) = camera_query.get_single_mut() {
			let player_x = player_transform.translation.x;
			let camera_x = camera_transform.translation.x;
			let left_bound = camera_x - (CONFIG.window_width / 2.) + 100.0;
			let right_bound = camera_x + (CONFIG.window_width / 2.) - 100.0;

			if player_x > right_bound {
				camera_transform.translation.x += player_x - right_bound;
			} else if player_x < left_bound {
				camera_transform.translation.x += player_x - left_bound;
			}
		}
	}
}
