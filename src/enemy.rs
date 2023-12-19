use crate::prelude::*;
use crate::game::in_gameplay;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<EnemySpawnTimer>()
			.add_systems(Startup, spawn_enemies)
			.add_systems(FixedUpdate, enemy_movement)
			.add_systems(Update, (
				enemy_damage_player,
				tick_enemy_spawn_timer,
				spawn_enemies_over_time,
				enemy_death_check,
			).run_if(in_gameplay));
	}
}

pub fn spawn_enemies(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut global_rng: ResMut<GlobalRng>,
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	for _ in 0..5 {
		let x = global_rng.f32_normalized() * primary.width();
		let y = global_rng.f32_normalized() * primary.height();
		commands.spawn(
			(
				SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					sprite: TextureAtlasSprite::new(1),
					transform: Transform::from_xyz(x, y, 0.),
					..Default::default()
				},
				Enemy {
					speed: 200.,
					hp: 10.,
					asset: 1usize,
					dmg: 10.,
				},
				RigidBody::Dynamic,
				Collider::cuboid(GRID_SIZE / 2., GRID_SIZE / 2.),
				LockedAxes::ROTATION_LOCKED_Z,
				GamePlayEntity,
			)
		).with_children(|parent| {
			parent.spawn(SpriteBundle {
				transform: Transform::from_xyz(0., -GRID_SIZE, 0.),
				sprite: Sprite {
					color: Color::rgb(1., 1., 0.1),
					custom_size: Some(Vec2::new(28., 3.)),
					..default()
				},
				..default()
			});
		});
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
	for (mut transform, enemy) in enemy_query.iter_mut() {
		// Move toward the player
		let direction = Vec3::new(player_transform.translation.x - transform.translation.x, player_transform.translation.y - transform.translation.y, 0.).normalize();
		// enemy.direction = ;
		transform.translation += direction * enemy.speed * time.delta_seconds();
	}
}

pub fn enemy_damage_player(
	// mut game_over_event_writer: EventWriter<GameOver>,
	enemies_query: Query<(&Collider, &GlobalTransform, &Enemy)>,
	mut player_query: Query<&mut Player>,
	rapier_context: Res<RapierContext>,
	time: Res<Time>,
	// score: Res<Score>,
) {
	for (collider, transform, enemy) in &enemies_query {
		rapier_context.intersections_with_shape(
			transform.translation().truncate(),
			0.,
			collider,
			QueryFilter::new(),
			|e| {
				if let Ok(mut player) = player_query.get_mut(e) {
					player.hp -= enemy.dmg * time.delta_seconds();
				}
				true
			}
		);
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
	mut global_rng: ResMut<GlobalRng>,
) {
	let Ok(primary) = window_query.get_single() else {
		return;
	};
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	if enemy_spawn_timer.timer.finished() {
		let x = global_rng.f32_normalized() * primary.width();
		let y = global_rng.f32_normalized() * primary.height();
		commands.spawn(
			(
				SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					sprite: TextureAtlasSprite::new(1),
					transform: Transform::from_xyz(x, y, 0.),
					..Default::default()
				},
				Enemy {
					speed: 200.,
					hp: 10.,
					asset: 1usize,
					dmg: 10.,
				},
				RigidBody::Dynamic,
				Collider::cuboid(GRID_SIZE / 2., GRID_SIZE / 2.),
				LockedAxes::ROTATION_LOCKED_Z,
				GamePlayEntity,
			)
		).with_children(|parent| {
			parent.spawn(SpriteBundle {
				transform: Transform::from_xyz(0., -GRID_SIZE, 0.),
				sprite: Sprite {
					color: Color::rgb(1., 1., 0.1),
					custom_size: Some(Vec2::new(28., 3.)),
					..default()
				},
				..default()
			});
		});
	}
}

pub fn enemy_death_check(
	mut commands: Commands,
	enemies: Query<(Entity, &Transform, &Enemy)>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

	for (entity, transform, enemy) in &enemies {
		if enemy.hp <= 0. {
			commands.entity(entity).despawn_recursive();
			commands.spawn(
				(
					SpriteSheetBundle {
						texture_atlas: texture_atlas_handle.clone(),
						sprite: TextureAtlasSprite::new(3),
						transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.),
						..Default::default()
					},
					Exp {
						value: 1,
						speed: 50.,
						collecting: false,
					},
					Sensor,
					Collider::ball(GRID_SIZE / 2.),
					GamePlayEntity,
				)
			);
		}
	}
}