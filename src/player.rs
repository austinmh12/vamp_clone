use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, spawn_player)
			.add_systems(FixedUpdate, player_movement)
			.add_systems(Update, (confine_player_movement, player_game_over));
	}
}

pub fn spawn_player(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.spawn(
		(
			SpriteSheetBundle {
				texture_atlas: texture_atlas_handle,
				sprite: TextureAtlasSprite::new(0),
				transform: Transform::from_xyz(primary.width() / 2., primary.height() / 2., 0.),
				..default()
			},
			Player {
				exp: 0,
				next_lvl_exp: 5,
				level: 1,
				speed: 500.,
				hp: 100.,
				max_hp: 100.,
			},
			Name::new("Player"),
			Collider::ball(GRID_SIZE),
		)
	);
}

pub fn player_movement(
	keyboard_input: Res<Input<KeyCode>>,
	mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
	time: Res<Time>,
) {
	let Ok((mut transform, player)) = player_query.get_single_mut() else {
		return;
	};
	let mut direction = Vec3::ZERO;
	// Left Right
	if keyboard_input.pressed(KeyCode::A) {
		direction += Vec3::new(-1., 0., 0.)
	}
	if keyboard_input.pressed(KeyCode::D) {
		direction += Vec3::new(1., 0., 0.)
	}
	// Up Down
	if keyboard_input.pressed(KeyCode::W) {
		direction += Vec3::new(0., 1., 0.)
	}
	if keyboard_input.pressed(KeyCode::S) {
		direction += Vec3::new(0., -1., 0.)
	}

	if direction.length() > 0. {
		direction = direction.normalize();
	}

	transform.translation += direction * player.speed * time.delta_seconds();
}

// TODO: Remove this as the camera should follow the player
pub fn confine_player_movement(
	mut player_query: Query<&mut Transform, With<Player>>,
	window_query: Query<&Window, With<PrimaryWindow>>,
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let Ok(mut transform) = player_query.get_single_mut() else {
		return;
	};
	let half_player_width = GRID_SIZE / 2.;
	let half_player_height = GRID_SIZE / 2.;
	let x_min = 0. + half_player_width;
	let x_max = primary.width() - half_player_width;
	let y_min = 0. + half_player_height;
	let y_max = primary.height() - half_player_height;

	// Confine X
	if transform.translation.x < x_min {
		transform.translation.x = x_min;
	} else if transform.translation.x > x_max {
		transform.translation.x = x_max;
	}

	// Confine Y
	if transform.translation.y < y_min {
		transform.translation.y = y_min;
	} else if transform.translation.y > y_max {
		transform.translation.y = y_max;
	}
}

pub fn player_game_over(
	player_query: Query<&Player>,
	mut game_state: ResMut<NextState<GameState>>,
) {
	let Ok(player) = player_query.get_single() else {
		return;
	};
	if player.hp <= 0. {
		game_state.set(GameState::GameOver);
	}
}