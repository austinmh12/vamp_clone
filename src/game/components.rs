use bevy::prelude::*;

pub type Currency = u128;

#[derive(Resource)]
pub struct Mora {
	pub mora: Currency,
}

impl Default for Mora {
	fn default() -> Self {
		Self { mora: 0 }
	}
}

#[derive(Component)]
pub struct Wallet(pub Currency);

