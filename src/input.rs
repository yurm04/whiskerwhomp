use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
	character::Velocity,
	movement::Jump,
	player::{Player, PLAYER_CONFIG},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, movement_input).add_systems(Update, jump_input);
	}
}

fn movement_input(
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut player_query: Query<&mut KinematicCharacterController>,
	velocity_query: Query<&Velocity, With<Player>>,
) {
	let mut player = player_query.single_mut();
	let velocity = velocity_query.single();

	let mut movement = 0.0;

	if input.pressed(KeyCode::ArrowRight) {
		movement += time.delta_seconds() * velocity.x;
	}

	if input.pressed(KeyCode::ArrowLeft) {
		movement += time.delta_seconds() * velocity.x * -1.0;
	}

	if let Some(mut translation) = player.translation {
		translation.x = movement;
		player.translation = Some(translation);
	} else {
		player.translation = Some(Vec2::new(movement, 0.0));
	}
}

#[allow(clippy::type_complexity)]
fn jump_input(
	input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	query: Query<
		(Entity, &KinematicCharacterControllerOutput),
		With<KinematicCharacterController>,
	>,
) {
	if query.is_empty() {
		return;
	}

	let (player, output) = query.single();

	if input.pressed(KeyCode::ArrowUp) && output.grounded {
		commands.entity(player).insert(Jump {
			total: 0.0,
			duration: PLAYER_CONFIG.jump_duration,
			max_height: PLAYER_CONFIG.max_jump_height,
		});
	}
}
