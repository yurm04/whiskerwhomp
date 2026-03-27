use avian2d::prelude::*;
use bevy::prelude::*;

use crate::CONFIG;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

fn spawn_platform(commands: &mut Commands, x: f32, width: f32, height: f32) {
	commands.spawn((
		Sprite::from_color(COLOR_PLATFORM, Vec2::new(width, height)),
		Transform::from_xyz(
			x,
			CONFIG.window_bottom_y + (height / 2.0) + CONFIG.floor_thickness,
			1.0,
		),
		RigidBody::Static,
		Collider::rectangle(width, height),
	));
}

fn setup(mut commands: Commands) {
	spawn_platform(&mut commands, -100.0, 75.0, 50.0);
	spawn_platform(&mut commands, 100.0, 50.0, 60.0);
	spawn_platform(&mut commands, 350.0, 150.0, 30.0);
}
