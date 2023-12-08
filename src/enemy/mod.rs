pub mod components;
pub mod systems;
pub mod resources;

pub const ENEMY_COUNT: usize = 4;
pub const ENEMY_SPEED: f32 = 200.;
const ENEMY_SPAWN_TIME: f32 = 5.;

use bevy::prelude::*;

use self::{resources::EnemySpawnTimer, systems::{enemy_movement, spawn_enemies, tick_enemy_spawn_timer, enemy_hit_player, spawn_enemies_over_time}};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<EnemySpawnTimer>()
			.add_systems(Startup, spawn_enemies)
			.add_systems(FixedUpdate, enemy_movement)
			.add_systems(Update, (enemy_hit_player, tick_enemy_spawn_timer, spawn_enemies_over_time));
	}
}