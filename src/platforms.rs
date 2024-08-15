use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::CONFIG;

const COLOR_PLATFORM: Color = Color::srgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
struct PlatformBundle {
	sprite_bundle: SpriteBundle,
	body: RigidBody,
	collider: Collider,
}

impl PlatformBundle {
	fn new(x: f32, width: f32, height: f32) -> Self {
		Self {
			sprite_bundle: SpriteBundle {
				sprite: Sprite {
					color: COLOR_PLATFORM,
					custom_size: Some(Vec2::new(width, height)),
					..Default::default()
				},
				transform: Transform {
					translation: Vec3::new(
						x,
						CONFIG.window_bottom_y + (height / 2.0) + CONFIG.floor_thickness,
						1.0,
					),
					scale: Vec3::new(1.0, 1.0, 1.0),
					..Default::default()
				},
				..Default::default()
			},
			body: RigidBody::Fixed,
			collider: Collider::cuboid(width / 2.0, height / 2.0),
		}
	}
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

fn setup(mut commands: Commands) {
	commands.spawn(PlatformBundle::new(-100.0, 75.0, 200.0));
	commands.spawn(PlatformBundle::new(100.0, 50.0, 350.0));
	commands.spawn(PlatformBundle::new(350.0, 150.0, 220.0));
}
