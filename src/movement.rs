use avian2d::prelude::*;
use bevy::prelude::*;

use crate::character::{GroundSensor, Grounded};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, ground_detection);
	}
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
