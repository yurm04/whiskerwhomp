use bevy::{
	prelude::*,
	window::{Window, WindowPlugin, WindowResolution},
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Component)]
struct Player;

fn move_player(
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	window: Single<&Window>,
	mut player_transform: Single<&mut Transform, With<Player>>,
) {
	let mut direction = Vec2::ZERO;

	if input.pressed(KeyCode::ArrowLeft) {
		direction.x -= 1.0;
	}
	if input.pressed(KeyCode::ArrowRight) {
		direction.x += 1.0;
	}
	if input.pressed(KeyCode::ArrowUp) {
		direction.y += 1.0;
	}
	if input.pressed(KeyCode::ArrowDown) {
		direction.y -= 1.0;
	}

	if direction != Vec2::ZERO {
		let speed = 300.0;
		let max_x = window.width() / 2.0;
		let max_y = window.height() / 2.0;
		let delta = direction.normalize() * speed * time.delta_secs();
		player_transform.translation.x =
			(player_transform.translation.x + delta.x).clamp(-max_x, max_x);
		player_transform.translation.y =
			(player_transform.translation.y + delta.y).clamp(-max_y, max_y);
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Whisker Whomper".into(),
				resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
				resizable: true,
				..default()
			}),
			..default()
		}))
		.add_systems(Startup, setup)
		.add_systems(Update, move_player)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);

	commands.spawn((
		Text2d::new("ME!"),
		TextFont {
			font_size: 12.0,
			font: default(),
			..default()
		},
		TextColor(Color::WHITE),
		Transform::from_translation(Vec3::ZERO),
		Player,
	));
}
