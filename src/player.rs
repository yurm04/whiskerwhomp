use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};

use crate::{CONFIG, animation::Animation};

const MOVE_SPEED: f32 = 400.0;
const JUMP_SPEED: f32 = 700.0;

const SPRITESHEET_COLS: u32 = 8;
const SPRITESHEET_ROWS: u32 = 10;
const SPRITE_PATH: &str = "spritesheets/cat_sprite.png";
const SPRITE_TILE_SIZE: u32 = 32;
const SPRITE_RENDER_SIZE: f32 = 64.0;
const CYCLE_DELAY: Duration = Duration::from_millis(70);

const FRAMES_IDLE: &[&[usize]] = &[
	&[0, 1, 2, 3],     // tail wiggle
	&[8, 9, 10, 11],   // look around
	&[16, 17, 18, 19], // lick paw
	&[24, 25, 26, 27], // clean head
	&[48, 49, 50, 51], // sleep
];
const IDLE_CHANGEUP: Duration = Duration::from_secs(10);
const IDLE_RETURN: Duration = Duration::from_secs(15);
const CYCLE_DELAY_IDLE: Duration = Duration::from_millis(280);

const FRAMES_WALKING: &[usize] = &[32, 33, 34, 35, 36, 37, 38, 39, 40];
const FRAMES_JUMPING: &[usize] = &[64, 65, 66, 67, 68, 69, 70];

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
struct Grounded(bool);

#[derive(Component)]
struct GroundSensor;

#[derive(Component)]
struct IdleTimer {
	since: Instant,
	idle_animation: Option<usize>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, Self::setup).add_systems(
			Update,
			(Self::ground_detection, Self::handle_input, Self::update_animation),
		);
	}
}

impl PlayerPlugin {
	fn setup(
		mut commands: Commands,
		mut atlases: ResMut<Assets<TextureAtlasLayout>>,
		server: Res<AssetServer>,
	) {
		let image: Handle<Image> = server.load(SPRITE_PATH);
		let layout = TextureAtlasLayout::from_grid(
			UVec2::splat(SPRITE_TILE_SIZE),
			SPRITESHEET_COLS,
			SPRITESHEET_ROWS,
			None,
			None,
		);
		let atlas_handle = atlases.add(layout);

		commands
			.spawn((
				Player,
				IdleTimer {
					since: Instant::now(),
					idle_animation: None,
				},
				Sprite {
					image,
					custom_size: Some(Vec2::splat(SPRITE_RENDER_SIZE)),
					texture_atlas: Some(TextureAtlas {
						layout: atlas_handle,
						index: 0,
					}),
					..default()
				},
				Transform::from_xyz(
					CONFIG.window_left_x + 100.0,
					CONFIG.window_bottom_y + 300.0,
					1.0,
				),
				Animation::new(FRAMES_IDLE[0], CYCLE_DELAY),
				RigidBody::Dynamic,
				Collider::rectangle(SPRITE_RENDER_SIZE / 2.5, SPRITE_RENDER_SIZE),
				LockedAxes::ROTATION_LOCKED,
				LinearVelocity::ZERO,
				Grounded(false),
			))
			.with_children(|parent| {
				parent.spawn((
					Transform::from_xyz(0.0, -SPRITE_RENDER_SIZE / 2.0 - 2.0, 0.0),
					Collider::rectangle(SPRITE_RENDER_SIZE * 0.8, 4.0),
					Sensor,
					GroundSensor,
					CollidingEntities::default(),
				));
			});
	}

	fn ground_detection(
		sensor_query: Query<(&ChildOf, &CollidingEntities), With<GroundSensor>>,
		mut player_query: Query<&mut Grounded>,
	) {
		for (child_of, colliding) in sensor_query.iter() {
			if let Ok(mut grounded) = player_query.get_mut(child_of.parent()) {
				grounded.0 = !colliding.is_empty();
			}
		}
	}

	fn handle_input(
		input: Res<ButtonInput<KeyCode>>,
		mut query: Query<
			(&mut LinearVelocity, &mut Sprite, &Grounded, &mut IdleTimer),
			With<Player>,
		>,
	) {
		let Ok((mut lin_vel, mut sprite, grounded, mut idle_timer)) =
			query.single_mut()
		else {
			return;
		};

		// resetting the idle timer each time someone presses a button
		// even if the button doesn't do anything
		if input.get_just_pressed().count() > 0 {
			idle_timer.since = Instant::now();
			idle_timer.idle_animation = None;
		}

		let mut movement = 0.0;
		if input.pressed(KeyCode::ArrowRight) {
			movement += MOVE_SPEED;
		}
		if input.pressed(KeyCode::ArrowLeft) {
			movement -= MOVE_SPEED;
		}
		lin_vel.x = movement;

		if movement > 0.0 {
			sprite.flip_x = false;
		} else if movement < 0.0 {
			sprite.flip_x = true;
		}

		if input.pressed(KeyCode::ArrowUp) && grounded.0 {
			lin_vel.y = JUMP_SPEED;
		}
	}

	fn update_animation(
		mut query: Query<
			(&LinearVelocity, &Grounded, &mut Animation, &mut IdleTimer),
			With<Player>,
		>,
	) {
		let Ok((lin_vel, grounded, mut animation, mut idle_timer)) =
			query.single_mut()
		else {
			return;
		};

		let new_frames: &'static [usize] = if !grounded.0 {
			let i = if lin_vel.y > 400.0 {
				0
			} else if lin_vel.y > 200.0 {
				1
			} else if lin_vel.y > 50.0 {
				2
			} else if lin_vel.y > -50.0 {
				3
			} else if lin_vel.y > -200.0 {
				4
			} else if lin_vel.y > -400.0 {
				5
			} else {
				6
			};
			&FRAMES_JUMPING[i..=i]
		} else if lin_vel.x.abs() > 1.0 {
			FRAMES_WALKING
		} else {
			// the player has been idle for at least IDLE_CHANGEUP so let's change the animation up
			if idle_timer.since.elapsed() > IDLE_CHANGEUP
				&& idle_timer.idle_animation.is_none()
			{
				let index = rand::rng().random_range(1..=4);
				idle_timer.idle_animation = Some(index);
				FRAMES_IDLE[index]
			// Showing the alternate idle animation
			} else if let Some(index) = idle_timer.idle_animation
				&& idle_timer.since.elapsed() > IDLE_CHANGEUP
				&& idle_timer.since.elapsed() < IDLE_RETURN
			{
				FRAMES_IDLE[index]
			// resetting the idle animation for one frame only
			} else if idle_timer.idle_animation.is_some()
				&& idle_timer.since.elapsed() >= IDLE_RETURN
			{
				idle_timer.idle_animation = None;
				idle_timer.since = Instant::now();
				FRAMES_IDLE[0]
			// normal idle animation
			} else {
				idle_timer.idle_animation = None;
				FRAMES_IDLE[0]
			}
		};

		if !std::ptr::eq(animation.frames, new_frames) {
			animation.frames = new_frames;
			animation.current = 0;
		}

		// idle animations are slower or it looks like the cat is stressed
		animation.timer.set_duration(if grounded.0 && lin_vel.x.abs() <= 1.0 {
			CYCLE_DELAY_IDLE
		} else {
			CYCLE_DELAY
		});
	}
}
