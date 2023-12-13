use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::*;

use vamp_clone::prelude::*;

pub const GRID_SIZE: f32 = 31.;

fn main() {
    App::new()
		.add_state::<GameState>()
		.add_plugins((
			DefaultPlugins.set(ImagePlugin::default_nearest()),
			GamePlugin,
			EnemyPlugin,
			PlayerPlugin,
			ScorePlugin,
			SpellsPlugin,
			WorldInspectorPlugin::default(),
			RngPlugin::default(),
			RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.),
		))
		.register_type::<Player>()
		.insert_resource(RapierConfiguration {
			gravity: Vec2::ZERO,
			..default()
		})
		.add_systems(OnEnter(GameState::GameOver), despawn_game_play)
		.run();
}

fn despawn_game_play(mut commands: Commands, entities: Query<Entity, With<GamePlayEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}