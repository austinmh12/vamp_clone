use bevy::prelude::*;

use self::systems::*;
use crate::events::GameOver;

pub mod components;
pub mod resources;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_event::<GameOver>()
			.add_systems(Startup, spawn_camera)
			.add_systems(Update, (handle_game_over, exit_game));
	}
}