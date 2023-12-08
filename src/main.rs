use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;
mod player;
mod enemy;
mod score;
mod exp;
mod events;
mod spells;

use game::GamePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use spells::SpellsPlugin;

pub const GRID_SIZE: f32 = 31.;

fn main() {
    App::new()
		.add_plugins((
			DefaultPlugins.set(ImagePlugin::default_nearest()),
			GamePlugin,
			EnemyPlugin,
			PlayerPlugin,
			ScorePlugin,
			SpellsPlugin,
			WorldInspectorPlugin::default(),
		))
		.run();
}