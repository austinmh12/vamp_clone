use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;
use crate::events::GameOver;

pub fn exit_game(
	keyboard_input: Res<Input<KeyCode>>,
	mut app_exit_event_writer: EventWriter<AppExit>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		app_exit_event_writer.send(AppExit);	
	}
}


pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
	for event in game_over_event_reader.read() {
		println!("Score: {}", event.score);
	}
}

pub fn spawn_camera(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
) {
	let window = window_query.get_single().unwrap();

	commands.spawn(Camera2dBundle {
		transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
		..default()
	});
}