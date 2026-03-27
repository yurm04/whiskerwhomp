use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

mod animation;
mod camera;
mod player;
mod world;

use animation::AnimationPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub struct Config {
	pub window_width: u32,
	pub window_height: u32,
	pub window_bottom_y: f32,
	pub window_left_x: f32,
	pub floor_thickness: f32,
	color_background: Color,
	title: &'static str,
}

pub static CONFIG: Config = Config {
	window_width: 1024,
	window_height: 720,
	window_bottom_y: 720.0 / -2.0,
	window_left_x: 1024.0 / -2.0,
	floor_thickness: 5.0,
	color_background: Color::srgb(0.13, 0.13, 0.23),
	title: "Whiskerwhomp",
};

fn main() {
	App::new()
		.insert_resource(ClearColor(CONFIG.color_background))
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
			WindowPlugin {
				primary_window: Some(Window {
					title: CONFIG.title.to_string(),
					resolution: WindowResolution::new(
						CONFIG.window_width,
						CONFIG.window_height,
					),
					resizable: true,
					..Default::default()
				}),
				..Default::default()
			},
		))
		.add_plugins(PhysicsPlugins::default())
		.add_plugins(PhysicsDebugPlugin)
		.insert_resource(Gravity(Vec2::new(0.0, -3000.0)))
		.add_systems(Startup, disable_physics_debug)
		.add_systems(Update, toggle_physics_debug)
		.add_plugins((WorldPlugin, PlayerPlugin, AnimationPlugin, CameraPlugin))
		.run();
}

fn disable_physics_debug(mut store: ResMut<GizmoConfigStore>) {
	store.config_mut::<PhysicsGizmos>().0.enabled = false;
}

fn toggle_physics_debug(
	input: Res<ButtonInput<KeyCode>>,
	mut store: ResMut<GizmoConfigStore>,
) {
	if input.just_pressed(KeyCode::KeyD) {
		let config = &mut store.config_mut::<PhysicsGizmos>().0;
		config.enabled = !config.enabled;
	}
}
