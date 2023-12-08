use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use self::systems::*;
use self::resources::*;

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<SpellSpawnTimer>()
			.add_systems(Update, (spawn_spells_over_time, spell_hit_enemy, tick_spell_spawn_timer))
			.add_systems(FixedUpdate, spell_movement);
	}
}