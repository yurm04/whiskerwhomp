use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::PLAYER_CONFIG, CONFIG};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(Update, camera_follow_system);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
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
			let left_bound = camera_x - (CONFIG.window_width / 2.)
				+ PLAYER_CONFIG.camera_edge_boundary;
			let right_bound = camera_x + (CONFIG.window_width / 2.)
				- PLAYER_CONFIG.camera_edge_boundary;

			if player_x > right_bound {
				camera_transform.translation.x += player_x - right_bound;
			} else if player_x < left_bound {
				camera_transform.translation.x += player_x - left_bound;
			}
		}
	}
}
