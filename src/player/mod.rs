use bevy::prelude::*;

pub mod components;
pub mod systems;

use self::systems::*;

pub const PLAYER_SPEED: f32 = 500.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, spawn_player)
			.add_systems(FixedUpdate, player_movement)
			.add_systems(Update, confine_player_movement);
	}
}