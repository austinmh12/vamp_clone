use crate::prelude::*;
use crate::game::in_gameplay;

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<SpellSpawnTimer>()
			.add_systems(Update, (
				spawn_spells_over_time,
				spell_damage_enemy,
				tick_spell_spawn_timer
			).run_if(in_gameplay))
			.add_systems(FixedUpdate, spell_movement);
	}
}

pub fn _spawn_spell(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	player_query: Query<&Transform, With<Player>>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	let Ok(player_transform) = player_query.get_single() else {
		return;
	};
	let texture_handle = asset_server.load("sprites/spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
	// let mut direction = Vec3::ZERO;
	let mut prev_distance = 99999.;
	let mut closest_enemy = None;
	for enemy in enemy_query.iter() {
		let distance = player_transform.translation.distance(enemy.translation);
		if distance < prev_distance {
			prev_distance = distance;
			closest_enemy = Some(enemy);
		}
	}
	let closest_enemy = closest_enemy.unwrap();
	let direction = Vec2::new(closest_enemy.translation.x - player_transform.translation.x, closest_enemy.translation.y - player_transform.translation.y).normalize();
	commands.spawn(
		(
			SpriteSheetBundle {
				texture_atlas: texture_atlas_handle.clone(),
				sprite: TextureAtlasSprite::new(2),
				transform: Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, 0.),
				..Default::default()
			},
			Spell {
				timer: Timer::from_seconds(1., TimerMode::Repeating),
				damage: 2.,
				speed: 100.,
				direction,
			},
		)
	);
}

pub fn spell_movement(
	mut spell_query: Query<(&mut Transform, &Spell), Without<Player>>,
	time: Res<Time>,
) {
	for (mut transform, spell) in spell_query.iter_mut() {
		let direction = Vec3::new(spell.direction.x, spell.direction.y, 0.).normalize();
		transform.translation += direction * spell.speed * time.delta_seconds();
	}
}

pub fn spell_damage_enemy(
	spell_query: Query<(&Collider, &GlobalTransform, &Spell)>,
	mut enemy_query: Query<(&mut Enemy, &Children)>,
	mut hp_bar_query: Query<&mut Sprite>,
	rapier_context: Res<RapierContext>,
	time: Res<Time>,
) {
	for (spell_collider, spell_transform, spell) in &spell_query {
		rapier_context.intersections_with_shape(
			spell_transform.translation().truncate(),
			0.,
			spell_collider,
			QueryFilter::new(),
			|e| {
				if let Ok((mut enemy, children)) = enemy_query.get_mut(e) {
					enemy.hp -= spell.damage * time.delta_seconds();
					for child in children {
						if let Ok(mut hp_bar) = hp_bar_query.get_mut(*child) {
							hp_bar.custom_size = Some(Vec2::new(28. * (enemy.hp / 10.), 3.))
						}
					}
				}
				true
			}
		);
	}
}

pub fn tick_spell_spawn_timer(
	mut spell_spawn_timer: ResMut<SpellSpawnTimer>,
	time: Res<Time>,
) {
	spell_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_spells_over_time(
	mut commands: Commands,
	player_query: Query<&Transform, With<Player>>,
	// enemy_query: Query<&Transform, With<Enemy>>,
	mouse_pos: Res<MousePosition>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	spell_spawn_time: Res<SpellSpawnTimer>,
) {
	let Ok(player_transform) = player_query.get_single() else {
		return;
	};
	if spell_spawn_time.timer.finished() {
		println!("Spawning spell");
		let texture_handle = asset_server.load("sprites/spritesheet.png");
		let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GRID_SIZE, GRID_SIZE), 20, 20, None, None);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);
		// let mut direction = Vec3::ZERO;
		// let mut prev_distance = 99999.;
		// let mut closest_enemy = None;
		// for enemy in enemy_query.iter() {
		// 	let distance = player_transform.translation.distance(enemy.translation);
		// 	if distance < prev_distance {
		// 		prev_distance = distance;
		// 		closest_enemy = Some(enemy);
		// 	}
		// }
		// if closest_enemy.is_none() {
		// 	return;
		// }
		// let closest_enemy = closest_enemy.unwrap();
		// let direction = Vec2::new(closest_enemy.translation.x - player_transform.translation.x, closest_enemy.translation.y - player_transform.translation.y).normalize();
		let direction = Vec2::new(mouse_pos.pos.x - player_transform.translation.x, mouse_pos.pos.y - player_transform.translation.y);
		let rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), direction.y.atan2(direction.x) - f32::to_radians(90.));
		let mut transform = Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, 0.);
		transform.rotation = rotation;
		commands.spawn(
			(
				SpriteSheetBundle {
					texture_atlas: texture_atlas_handle.clone(),
					sprite: TextureAtlasSprite::new(2),
					transform,
					..Default::default()
				},
				Spell {
					timer: Timer::from_seconds(0.3, TimerMode::Repeating),
					damage: 20.,
					speed: 250.,
					direction,
				},
				Sensor,
				Collider::ball(GRID_SIZE),
				GamePlayEntity,
			)
		);
	}
}