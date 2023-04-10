use bevy::prelude::*;
use crate::{AppState, MainCamera, UIFont};
use crate::cmp::Interactable;
use crate::scn::game::player::Player;

const INTERACT_RADIUS : f32 = 100.;
const INTERACT_RADIUS_2 : f32 = INTERACT_RADIUS * INTERACT_RADIUS;

#[derive(Component)]
struct InteractUIRoot;

pub struct InteractPlugin;
impl Plugin for InteractPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(setup.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown.in_schedule(OnExit(AppState::Game)))
			.add_system(toggle_interact_label.in_set(OnUpdate(AppState::Game)))
		;
	}
}

fn setup (
	mut commands : Commands,
	ui_font : Res<UIFont>,
) {
	let mut label = TextBundle::from_sections([
		TextSection::new(
			"[E] ",
			TextStyle {
				font: ui_font.bold.clone(),
				font_size: 32.,
				..default()
			},
		),
		TextSection::new(
			"Interact",
			TextStyle {
				font: ui_font.light.clone(),
				font_size: 32.,
				..default()
			},
		),
	])
		.with_text_alignment(TextAlignment::Center)
		.with_style(Style {
			position_type: PositionType::Absolute,
			position: UiRect::default(),
			..default()
		});
	
	label.visibility = Visibility::Hidden;
	
	commands.spawn((
		InteractUIRoot,
		label,
	));
}

fn teardown (
	mut commands : Commands,
	query: Query<Entity, With<InteractUIRoot>>,
) {
	commands.entity(query.single()).despawn_recursive();
}

fn toggle_interact_label (
	mut label_query : Query<(&mut Visibility, &mut Style), With<InteractUIRoot>>,
	player_query : Query<&GlobalTransform, With<Player>>,
	interactable_query : Query<(&GlobalTransform, &Interactable), Without<Player>>,
	camera_query : Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Interactable>, Without<Player>)>,
) {
	let mut label = label_query.single_mut();
	let player_pos = player_query.single().translation();
	let main_camera = camera_query.single();
	
	for (interactable_pos, interactable) in &interactable_query {
		let pos = interactable_pos.translation();
		let x = pos.x - player_pos.x;
		let y = pos.y - player_pos.y;
		
		if (x * x) + (y * y) > INTERACT_RADIUS_2 { continue }
		
		if let Some(coords) = main_camera.0.world_to_viewport(main_camera.1, pos) {
			*label.0 = Visibility::Visible;
			label.1.position = UiRect {
				left: Val::Px(coords.x + interactable.label_offset.x),
				bottom: Val::Px(coords.y + interactable.label_offset.y),
				..default()
			};
			
			return;
		}
	}
	
	*label.0 = Visibility::Hidden;
}
