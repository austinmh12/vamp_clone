use bevy::prelude::*;

#[derive(Component)]
pub struct Spell {
	pub direction: Vec2,
	pub speed: f32,
}