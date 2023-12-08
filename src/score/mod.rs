use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use self::resources::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<Score>();
	}
}