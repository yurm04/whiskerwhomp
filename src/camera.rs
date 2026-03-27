use bevy::prelude::*;

use crate::{CONFIG, player::Player};

const EDGE_BOUNDARY: f32 = 100.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
			.add_systems(Update, camera_follow);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
}

fn camera_follow(
	player_query: Query<&Transform, With<Player>>,
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
	if let Ok(player_transform) = player_query.single()
		&& let Ok(mut camera_transform) = camera_query.single_mut()
	{
		let player_x = player_transform.translation.x;
		let camera_x = camera_transform.translation.x;
		let half_width = (CONFIG.window_width / 2) as f32;
		let left = camera_x - half_width + EDGE_BOUNDARY;
		let right = camera_x + half_width - EDGE_BOUNDARY;

		if player_x > right {
			camera_transform.translation.x += player_x - right;
		} else if player_x < left {
			camera_transform.translation.x += player_x - left;
		}
	}
}
