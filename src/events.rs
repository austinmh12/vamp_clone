use crate::prelude::*;


#[derive(Event)]
pub struct GameOver {
	pub score: u32,
}

#[derive(Event)]
pub struct LevelUp {
	pub level: i64,
}