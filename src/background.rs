use bevy::prelude::*;

const BG_PATH: &str = "backgrounds/bg.png";

#[derive(Component)]
struct ParallaxLayer {
	factor: f32,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup).add_systems(Update, parallax_follow);
	}
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
	// Back layer: painted bedroom
	commands.spawn((
		Sprite::from_image(server.load(BG_PATH)),
		Transform::from_xyz(0.0, 0.0, -10.0),
		ParallaxLayer { factor: 0.2 },
	));

	// Mid layer placeholder (swap for real art later)
	commands.spawn((
		Sprite::from_color(
			Color::srgba(0.85, 0.2, 0.2, 0.35),
			Vec2::new(400.0, 400.0),
		),
		Transform::from_xyz(-200.0, -40.0, -9.0),
		ParallaxLayer { factor: 0.5 },
	));

	// Front layer placeholder (swap for real art later)
	commands.spawn((
		Sprite::from_color(
			Color::srgba(1.0, 0.35, 0.35, 0.45),
			Vec2::new(250.0, 250.0),
		),
		Transform::from_xyz(250.0, -100.0, -8.0),
		ParallaxLayer { factor: 0.8 },
	));
}

fn parallax_follow(
	camera_query: Query<&Transform, (With<Camera>, Without<ParallaxLayer>)>,
	mut layer_query: Query<(&ParallaxLayer, &mut Transform)>,
) {
	if let Ok(camera_transform) = camera_query.single() {
		let x = camera_transform.translation.x;
		for (layer, mut transform) in layer_query.iter_mut() {
			transform.translation.x = x * (1.0 - layer.factor);
		}
	}
}
