use bevy::prelude::*;

#[derive(Component)]
pub struct ParallaxLayer {
	pub factor: f32,
}

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, Self::parallax_follow);
	}
}

impl ParallaxPlugin {
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
}
