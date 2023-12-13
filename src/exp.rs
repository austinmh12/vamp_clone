use crate::prelude::*;

pub struct ExpPlugin;

impl Plugin for ExpPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(FixedUpdate, exp_move_to_player);
	}
}

pub fn exp_move_to_player(
	mut exp: Query<(&mut Transform, &Exp)>,
	player: Query<&Transform, (With<Player>, Without<Exp>)>,
	time: Res<Time>,
) {
	let Ok(player_transform) = player.get_single() else {
		return;
	};
	for (mut transform, xp) in &mut exp {
		if xp.collecting {
			let direction = (transform.translation.truncate() - player_transform.translation.truncate()).normalize();
			transform.translation -= (direction * time.delta_seconds() * xp.speed).extend(0.);
		}
	}
}