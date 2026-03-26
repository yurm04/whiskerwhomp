use bevy::prelude::*;

use crate::{CONFIG, player::PLAYER_CONFIG, player::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(Update, camera_follow_system);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
}

fn camera_follow_system(
	player_query: Query<&Transform, With<Player>>,
	mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
	if let Ok(player_transform) = player_query.single()
		&& let Ok(mut camera_transform) = camera_query.single_mut()
	{
		let player_x = player_transform.translation.x;
		let camera_x = camera_transform.translation.x;
		let left_bound = camera_x - (CONFIG.window_width / 2) as f32
			+ PLAYER_CONFIG.camera_edge_boundary;
		let right_bound = camera_x + (CONFIG.window_width / 2) as f32
			- PLAYER_CONFIG.camera_edge_boundary;

		if player_x > right_bound {
			camera_transform.translation.x += player_x - right_bound;
		} else if player_x < left_bound {
			camera_transform.translation.x += player_x - left_bound;
		}
	}
}
