use bevy::{ecs::query, prelude::*};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(GreetTimer(Timer::from_seconds(
				2.0,
				TimerMode::Repeating,
			)))
			.add_systems(Startup, add_people)
			.add_systems(Update, (update_people, greet_people).chain());
	}
}

fn main() {
	App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

fn hello_world() {
	println!("Hello, World!");
}

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Yuraima".to_string())));
	commands.spawn((Person, Name("Raquel".to_string())));
	commands.spawn((Person, Name("Artie".to_string())));
	commands.spawn((Person, Name("Dom".to_string())));
}

fn greet_people(
	time: Res<Time>,
	mut timer: ResMut<GreetTimer>,
	query: Query<&Name, With<Person>>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("Hello, {}", name.0)
		}

		println!("-----")
	}
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
	for mut name in &mut query {
		if name.0 == "Yuraima" {
			name.0 = "Yurms".to_string();
			break;
		}
	}
}
