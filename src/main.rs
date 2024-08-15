use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

mod animation;
mod platforms;
mod player;

use animation::AnimationPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;

pub struct Config {
	window_width: f32,
	window_height: f32,
	pub window_bottom_y: f32,
	pub window_left_x: f32,
	floor_thickness: f32,
	color_background: Color,
	color_floor: Color,
	title: &'static str,
}

pub static CONFIG: Config = Config {
	window_width: 1024.0,
	window_height: 720.0,
	window_bottom_y: 720.0 / -2.0,
	window_left_x: 1024.0 / -2.0,
	floor_thickness: 500.0,
	color_background: Color::srgb(0.13, 0.13, 0.23),
	color_floor: Color::srgb(0.45, 0.55, 0.66),
	title: "Whiskerwhomp",
};

fn main() {
	App::new()
		.insert_resource(ClearColor(CONFIG.color_background))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
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
		}))
		.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
		.add_plugins(RapierDebugRenderPlugin::default())
		.add_plugins(PlatformsPlugin)
		.add_plugins(PlayerPlugin)
		.add_plugins(AnimationPlugin)
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());

	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: CONFIG.color_floor,
				..Default::default()
			},
			transform: Transform {
				translation: Vec3::new(
					-500.0,
					CONFIG.window_bottom_y + (CONFIG.floor_thickness / 2.0),
					0.0,
				),
				scale: Vec3::new(
					CONFIG.window_width * 1000.0,
					CONFIG.floor_thickness,
					1.0,
				),
				..Default::default()
			},
			..Default::default()
		})
		.insert(RigidBody::Fixed)
		.insert(Collider::cuboid(0.5, 0.5));
}
