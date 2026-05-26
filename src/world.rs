use avian2d::prelude::*;
use bevy::prelude::*;

use crate::CONFIG;

const COLOR_FLOOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

fn setup(mut commands: Commands) {
	// Floor
	commands.spawn((
		Sprite::from_color(
			COLOR_FLOOR,
			Vec2::new((CONFIG.window_width * 100) as f32, CONFIG.floor_thickness),
		),
		Transform::from_xyz(
			0.0,
			CONFIG.window_bottom_y - CONFIG.floor_thickness,
			1.0,
		),
		RigidBody::Static,
		Collider::rectangle(
			(CONFIG.window_width * 100) as f32,
			CONFIG.floor_thickness,
		),
	));
}
