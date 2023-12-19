use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, spawn_player_exp_bar_ui)
			.add_systems(Update, update_player_exp_bar);
	}
}

pub fn spawn_player_exp_bar_ui(mut commands: Commands) {
	let parent_node = (
		NodeBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(5.),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::FlexStart,
				flex_direction: FlexDirection::Row,
				position_type: PositionType::Absolute,
				..default()
			},
			background_color: BackgroundColor(Color::rgb(0.1, 0.33, 0.25)),
			..default()
		},
		HeaderUI,
		Name::new("Header Bar"),
	);

	let exp_node = (
		NodeBundle {
			style: Style {
				width: Val::Percent(0.),
				height: Val::Percent(100.),
				..default()
			},
			background_color: BackgroundColor(Color::rgb(0.1, 1., 0.25)),
			..default()
		},
		ExpUI,
		Name::new("Exp Bar"),
	);
	let text = (
		TextBundle {
			text: Text { 
				sections: vec![
					TextSection {
						value: "1".to_string(),
						style: TextStyle {
							font_size: 24.,
							color: Color::BLACK,
							..default()
						},
					}
				],
				alignment: TextAlignment::Center,
				..default()
			},
			style: Style {
				width: Val::Percent(50.),
				height: Val::Percent(50.),
				position_type: PositionType::Absolute,
				align_self: AlignSelf::Center,
				..default()
			},
			..default()
		},
		// TextBundle::from_section(
		// 	"1",
		// 	TextStyle {
		// 		font_size: 24.,
		// 		color: Color::BLACK,
		// 		..default()
		// 	},
		// ),
		ExpLevelUI,
		Name::new("Exp Level Text"),
	);

	commands.spawn(parent_node).with_children(|p| {
		p.spawn(exp_node); 
		p.spawn(text); 
	});
}

pub fn update_player_exp_bar(
	mut ui: Query<&mut Style, With<ExpUI>>,
	mut ui_text: Query<&mut Text, With<ExpLevelUI>>,
	player: Query<&Player>,
) {
	let Ok(mut style) = ui.get_single_mut() else {
		return;
	};
	let Ok(mut text) = ui_text.get_single_mut() else {
		return;
	};
	let Ok(player) = player.get_single() else {
		return;
	};
	style.width = Val::Percent((player.exp as f32 / player.next_lvl_exp as f32) * 100.);
	text.sections[0] = TextSection::new(
		format!("{}", player.level), 
		TextStyle {
			font_size: 24.,
			color: Color::BLACK,
			..default()
		}
	);
}