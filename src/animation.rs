use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
	pub frames: &'static [usize],
	pub current: usize,
	pub timer: Timer,
}

impl Animation {
	pub fn new(frames: &'static [usize], delay: Duration) -> Self {
		Self {
			frames,
			current: 0,
			timer: Timer::new(delay, TimerMode::Repeating),
		}
	}
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, Self::animate);
	}
}

impl AnimationPlugin {
	fn animate(mut query: Query<(&mut Sprite, &mut Animation)>, time: Res<Time>) {
		for (mut sprite, mut animation) in query.iter_mut() {
			if animation.timer.tick(time.delta()).just_finished()
				&& let Some(ref mut atlas) = sprite.texture_atlas
			{
				animation.current = (animation.current
					+ animation.timer.times_finished_this_tick() as usize)
					% animation.frames.len();
				atlas.index = animation.frames[animation.current];
			}
		}
	}
}
