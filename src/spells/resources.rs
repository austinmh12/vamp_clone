use bevy::prelude::*;

pub const SPELL_SPAWN_TIME: f32 = 1.;

#[derive(Resource)]
pub struct SpellSpawnTimer {
	pub timer: Timer
}

impl Default for SpellSpawnTimer {
	fn default() -> Self {
		Self {
			timer: Timer::from_seconds(SPELL_SPAWN_TIME, TimerMode::Repeating)
		}
	}
}