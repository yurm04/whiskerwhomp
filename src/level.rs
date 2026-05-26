use avian2d::prelude::*;
use bevy::prelude::*;

use crate::CONFIG;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

#[derive(Component)]
pub struct ParallaxLayer {
	pub factor: f32,
}

#[derive(Debug)]
enum Level {
	One,
	Two,
	Three,
	Four,
}

impl Level {
	fn get_config(&self) -> LevelConfig {
		match self {
			Level::One => LevelConfig {
				level: 1,
				name: String::from("Level 1"),
				platforms: vec![
					(-100.0, 75.0, 50.0),
					(100.0, 50.0, 60.0),
					(350.0, 150.0, 30.0),
				],
				mid_1: vec![(400.0, 400.0, 0.0)],
				mid_2: vec![(250.0, 250.0, 0.0)],
				bg: String::from("backgrounds/bg.png"),
			},
			Level::Two => LevelConfig {
				level: 2,
				name: String::from("Level 2"),
				platforms: Vec::new(),
				mid_1: Vec::new(),
				mid_2: Vec::new(),
				bg: String::from(""),
			},
			Level::Three => LevelConfig {
				level: 3,
				name: String::from("Level 3"),
				platforms: Vec::new(),
				mid_1: Vec::new(),
				mid_2: Vec::new(),
				bg: String::from(""),
			},
			Level::Four => LevelConfig {
				level: 4,
				name: String::from("Level 4"),
				platforms: Vec::new(),
				mid_1: Vec::new(),
				mid_2: Vec::new(),
				bg: String::from(""),
			},
		}
	}
}

#[derive(Debug)]
struct LevelConfig {
	level: u8,
	name: String,
	platforms: Vec<(f32, f32, f32)>,
	mid_1: Vec<(f32, f32, f32)>,
	mid_2: Vec<(f32, f32, f32)>,
	bg: String,
}

#[derive(Resource)]
struct LevelState {
	current_level: Level,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(PreStartup, LevelPlugin::setup)
			.add_systems(Startup, LevelPlugin::load_level);
	}
}

impl LevelPlugin {
	fn setup(mut commands: Commands) {
		commands.insert_resource(LevelState {
			current_level: Level::One,
		});
	}

	fn spawn_platform(commands: &mut Commands, x: f32, width: f32, height: f32) {
		commands.spawn((
			Sprite::from_color(COLOR_PLATFORM, Vec2::new(width, height)),
			Transform::from_xyz(x, CONFIG.window_bottom_y + (height / 2.0), 1.0),
			RigidBody::Static,
			Collider::rectangle(width, height),
		));
	}

	fn load_level(
		level_state: Res<LevelState>,
		mut commands: Commands,
		server: Res<AssetServer>,
	) {
		let level_config = level_state.current_level.get_config();

		// Background image
		commands.spawn((
			Sprite::from_image(server.load(level_config.bg)),
			Transform::from_xyz(0.0, 0.0, -10.0),
			ParallaxLayer { factor: 0.2 },
		));

		// Platforms
		for platform in &level_config.platforms {
			Self::spawn_platform(&mut commands, platform.0, platform.1, platform.2);
		}

		// Mid layer
		for mid_1 in &level_config.mid_1 {
			commands.spawn((
				Sprite::from_color(
					Color::srgba(0.85, 0.2, 0.2, 0.35),
					Vec2::new(mid_1.0, mid_1.1),
				),
				ParallaxLayer { factor: 0.5 },
			));
		}

		// Front layer
		for mid_2 in &level_config.mid_2 {
			commands.spawn((
				Sprite::from_color(
					Color::srgba(1.0, 0.35, 0.35, 0.45),
					Vec2::new(mid_2.0, mid_2.1),
				),
				ParallaxLayer { factor: 0.8 },
			));
		}
	}
}
