mod enemy;
mod exp;
mod game;
mod player;
mod score;
mod spells;
mod events;

pub mod prelude {
	pub const GRID_SIZE: f32 = 31.;

	pub use crate::enemy::EnemyPlugin;
	pub use crate::exp::ExpPlugin;
	pub use crate::game::GamePlugin;
	pub use crate::player::PlayerPlugin;
	pub use crate::score::ScorePlugin;
	pub use crate::spells::SpellsPlugin;

	pub use bevy::prelude::*;
	pub use bevy::window::PrimaryWindow;
	use bevy::time::Stopwatch;
	pub use bevy_rapier2d::prelude::*;
	pub use bevy_turborand::prelude::*;

	#[derive(Debug, Clone, States, PartialEq, Eq, Default, Hash)]
	pub enum GameState {
		#[default]
		MainMenu,
		// StartingLoop,
		Gameplay,
		LevelUp,
		GameOver,
	}

	#[derive(Component, Reflect)]
	pub struct Player {
		pub exp: i64,
		pub next_lvl_exp: i64,
		pub level: i64,
		pub speed: f32,
		pub hp: f32,
		pub max_hp: f32,
	}

	#[derive(Component, Clone)]
	pub struct Enemy {
		pub speed: f32,
		pub hp: f32,
		pub asset: usize,
		pub dmg: f32,
	}

	#[derive(Resource, Default)]
	pub struct CursorPosition {
		pub screen_position: Vec2,
	}

	#[derive(Bundle)]
	pub struct ExpBundle {
		pub sprite: SpriteBundle,
		pub exp: Exp,
		pub collider: Collider,
		pub game_play: GamePlayEntity,
		pub sensor: Sensor,
	}

	#[derive(Component)]
	pub struct Exp {
		pub value: i64,
		pub speed: f32,
		pub collecting: bool,
	}

	#[derive(Component)]
	pub struct GamePlayEntity;

	#[derive(Component)]
	pub struct Spell {
		pub timer: Timer,
		pub damage: f32,
		pub speed: f32,
		pub direction: Vec2,
	}

	// Here until I figure out waves
	#[derive(Resource)]
	pub struct EnemySpawnTimer {
		pub timer: Timer
	}

	impl Default for EnemySpawnTimer {
		fn default() -> Self {
			Self {
				timer: Timer::from_seconds(0.5, TimerMode::Repeating)
			}
		}
	}

	#[derive(Resource)]
	pub struct SpellSpawnTimer {
		pub timer: Timer
	}

	impl Default for SpellSpawnTimer {
		fn default() -> Self {
			Self {
				timer: Timer::from_seconds(1., TimerMode::Repeating)
			}
		}
	}
}