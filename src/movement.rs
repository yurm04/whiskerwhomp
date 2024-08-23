use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::character::Velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, rise).add_systems(Update, fall);
	}
}

#[derive(Component)]
pub struct Jump {
	pub total: f32,
	pub duration: f32,
	pub max_height: f32,
}

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

fn rise(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
	if query.is_empty() {
		return;
	}

	let (entity, mut character, mut jump) = query.single_mut();

	let jump_duration = jump.duration;
	let jump_height = jump.max_height;

	// Calculate the time passed as a fraction of the total jump duration
	let t = jump.total / jump_duration;
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
	match character.translation {
		Some(vec) => character.translation = Some(Vec2::new(vec.x, new_height)),
		None => character.translation = Some(Vec2::new(0.0, new_height)),
	}

	// Update the jump timer
	jump.total += time.delta().as_secs_f32();
}

fn fall(
	time: Res<Time>,
	mut character_query: Query<&mut KinematicCharacterController, Without<Jump>>,
	velocity_query: Query<&Velocity>,
) {
	if character_query.is_empty() {
		return;
	}

	let mut player = character_query.single_mut();
	let velocity = velocity_query.single();
	let movement = time.delta().as_secs_f32() * (velocity.y / 1.5) * -1.0;

	match player.translation {
		Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
		None => player.translation = Some(Vec2::new(0.0, movement)),
	}
}
