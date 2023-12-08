use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use crate::GRID_SIZE;
use super::{ENEMY_COUNT, ENEMY_SPEED};
use super::components::Enemy;
use super::resources::EnemySpawnTimer;
use crate::player::components::Player;
use crate::events::GameOver;
use crate::score::resources::Score;

pub fn spawn_enemies(
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

	for _ in 0..ENEMY_COUNT {
		let x = random::<f32>() * primary.width();
		let y = random::<f32>() * primary.height();
		commands.spawn(
			(
				SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					sprite: TextureAtlasSprite::new(1),
					transform: Transform::from_xyz(x, y, 0.),
					..Default::default()
				},
				Enemy {
					direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
				},
			)
		);
	}
}

pub fn enemy_movement(
	mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
	player_query: Query<&Transform, With<Player>>,
	time: Res<Time>,
) {
	let Ok(player_transform) = player_query.get_single() else {
		return;
	};
	for (mut transform, _enemy) in enemy_query.iter_mut() {
		// Move toward the player
		let direction = Vec3::new(player_transform.translation.x - transform.translation.x, player_transform.translation.y - transform.translation.y, 0.).normalize();
		// enemy.direction = ;
		transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
	}
}

pub fn enemy_hit_player(
	mut commands: Commands,
	mut game_over_event_writer: EventWriter<GameOver>,
	mut player_query: Query<(Entity, &Transform), With<Player>>,
	enemy_query: Query<&Transform, With<Enemy>>,
	_asset_server: Res<AssetServer>,
	score: Res<Score>,
) {
	let Ok((player_entity, player_transform)) = player_query.get_single_mut() else {
		return;
	};
	for enemy in enemy_query.iter() {
		let distance = player_transform.translation.distance(enemy.translation);
		if distance < GRID_SIZE {
			println!("Game Over!");
			commands.entity(player_entity).despawn();
			game_over_event_writer.send(GameOver { score: score.value });
		}
	}
}

pub fn tick_enemy_spawn_timer(
	mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
	time: Res<Time>,
) {
	enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time (
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	if enemy_spawn_timer.timer.finished() {
		let x = random::<f32>() * primary.width();
		let y = random::<f32>() * primary.height();
		commands.spawn(
			(
				SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					sprite: TextureAtlasSprite::new(1),
					transform: Transform::from_xyz(x, y, 0.),
					..Default::default()
				},
				Enemy {
					direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
				},
			)
		);
	}
}