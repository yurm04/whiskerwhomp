use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
	character::{Grounded, Velocity},
	movement::Jump,
	player::Player,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, movement_input).add_systems(Update, jump_input);
	}
}

fn movement_input(
	input: Res<ButtonInput<KeyCode>>,
	mut query: Query<(&mut LinearVelocity, &Velocity), With<Player>>,
) {
	if let Ok((mut lin_vel, velocity)) = query.single_mut() {
		let mut movement = 0.0;

		if input.pressed(KeyCode::ArrowRight) {
			movement += velocity.x;
		}

		if input.pressed(KeyCode::ArrowLeft) {
			movement -= velocity.x;
		}

		lin_vel.x = movement;
	}
}

fn jump_input(
	input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	query: Query<(Entity, &Transform, &Grounded), With<Player>>,
) {
	if query.is_empty() {
		return;
	}

	let Ok((player, transform, grounded)) = query.single() else {
		return;
	};

	if input.pressed(KeyCode::ArrowUp) && grounded.0 {
		commands.entity(player).insert(Jump {
			start_y: transform.translation.y,
			max_height: 550.,
		});
	}
}
