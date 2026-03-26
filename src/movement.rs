use avian2d::prelude::*;
use bevy::prelude::*;

use crate::character::{GroundSensor, Grounded, Velocity};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, (ground_detection, rise, fall).chain());
	}
}

#[derive(Component)]
pub struct Jump {
	pub start_y: f32,
	pub max_height: f32,
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

fn rise(
	mut commands: Commands,
	mut query: Query<(Entity, &Transform, &mut LinearVelocity, &Jump, &Velocity)>,
) {
	let Ok((entity, transform, mut lin_vel, jump, velocity)) = query.single_mut()
	else {
		return;
	};

	let current_height = transform.translation.y - jump.start_y;

	if current_height >= jump.max_height {
		commands.entity(entity).remove::<Jump>();
		lin_vel.y = 0.0;
	} else {
		lin_vel.y = velocity.y;
	}
}

fn fall(
	mut query: Query<(&mut LinearVelocity, &Velocity, &Grounded), Without<Jump>>,
) {
	let Ok((mut lin_vel, velocity, grounded)) = query.single_mut() else {
		return;
	};

	if grounded.0 {
		// Only zero out if we're actually moving downward — don't fight
		// the solver when it's already resolved the contact.
		if lin_vel.y < 0.0 {
			lin_vel.y = 0.0;
		}
	} else {
		lin_vel.y = -(velocity.y / 1.5);
	}
}
