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
	pub max_height: f32,
}

fn rise(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
	velocity_query: Query<&Velocity>,
) {
	if query.is_empty() {
		return;
	}

	let (entity, mut character, mut jump) = query.single_mut();
	let velocity = velocity_query.single();

	let mut movement = time.delta().as_secs_f32() * velocity.y;

	if movement + jump.total >= jump.max_height {
		movement = jump.max_height - jump.total;
		commands.entity(entity).remove::<Jump>();
	}

	jump.total += movement;

	match character.translation {
		Some(vec) => character.translation = Some(Vec2::new(vec.x, movement)),
		None => character.translation = Some(Vec2::new(0.0, movement)),
	}
}

fn fall(
	time: Res<Time>,
	mut character_query: Query<&mut KinematicCharacterController, Without<Jump>>,
	velocity_query: Query<&Velocity>,
) {
	if character_query.is_empty() {
		return;
	}

	let mut character = character_query.single_mut();
	let velocity = velocity_query.single();
	let movement = time.delta().as_secs_f32() * (velocity.y / 1.5) * -1.0;

	match character.translation {
		Some(vec) => character.translation = Some(Vec2::new(vec.x, movement)),
		None => character.translation = Some(Vec2::new(0.0, movement)),
	}
}
