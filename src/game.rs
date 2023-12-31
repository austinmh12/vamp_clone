use crate::prelude::*;
use bevy::app::AppExit;
use crate::events::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_event::<GameOver>()
			.add_event::<LevelUp>()
			.add_systems(Startup, spawn_camera)
			.add_systems(Update, (handle_game_over, exit_game, mouse_position_update))
			.add_systems(FixedUpdate, camera_follow_player);
	}
}

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

	commands.spawn(
		(
			Camera2dBundle {
				transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
				..default()
			},
			MainCamera,
		)
	);
}

pub fn camera_follow_player(
	mut camera_query: Query<&mut Transform, With<MainCamera>>,
	player_query: Query<&GlobalTransform, With<Player>>,
) {
	let Ok(mut camera_transform) = camera_query.get_single_mut() else {
		return;
	};
	let Ok(player_transform) = player_query.get_single() else {
		return;
	};
	let player_translation = player_transform.translation().truncate();
	camera_transform.translation.x = player_translation.x;
	camera_transform.translation.y = player_translation.y;
}

pub fn mouse_position_update(
    mut mouse_position: ResMut<MousePosition>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let Ok((camera, camera_transform)) = camera_query.get_single() else {
		return;
	};
	if let Some(world_position) = primary.cursor_position()
		.and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
		.map(|ray| ray.origin.truncate())
	{
		mouse_position.pos = world_position;
	}
}

pub fn in_gameplay(state: Res<State<GameState>>) -> bool {
	return state.get() == &GameState::Gameplay
}