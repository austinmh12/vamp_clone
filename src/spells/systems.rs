use bevy::prelude::*;
use crate::player::components::Player;
use crate::enemy::components::Enemy;
use crate::score::resources::Score;
use super::components::Spell;
use crate::GRID_SIZE;

pub fn spawn_spell(
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
				sprite: TextureAtlasSprite::new(0),
				transform: Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, 0.),
				..Default::default()
			},
			Spell {
				direction,
				speed: 100.,
			},
		)
	);
}

pub fn spell_movement(
	mut spell_query: Query<(&mut Transform, &Spell), Without<Player>>,
	time: Res<Time>,
) {
	// for (mut transform, spell) in spell_query.iter_mut() {
	// 	let direction = Vec3::new(spell.direction.x, spell.direction.y, 0.).normalize();
	// 	transform.translation += direction * spell.speed * time.delta_seconds();
	// }
}

pub fn spell_hit_enemy(
	mut spell_query: Query<(&mut Transform, &Spell), With<Spell>>,
	mut enemy_query: Query<(&mut Transform, &Enemy), With<Enemy>>,
	time: Res<Time>,
	mut score: ResMut<Score>,
) {

}