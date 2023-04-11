use bevy::prelude::*;
use crate::{AppState, MainCamera, UIFont};
use crate::cmp::{ActiveInteractable, Interactable, InteractableState};
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
			.add_systems((
				toggle_interact_label,
				handle_interact,
			).in_set(OnUpdate(AppState::Game)))
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
	mut commands : Commands,
	mut label_query : Query<(&mut Visibility, &mut Style), With<InteractUIRoot>>,
	player_query : Query<&GlobalTransform, With<Player>>,
	interactable_query : Query<(Entity, &GlobalTransform, &Interactable, Option<&ActiveInteractable>), Without<Player>>,
	camera_query : Query<(&Camera, &GlobalTransform), (With<MainCamera>, Without<Interactable>, Without<Player>)>,
) {
	let mut label = label_query.single_mut();
	let player_pos = player_query.single().translation();
	let main_camera = camera_query.single();
	
	for (
		entity, interactable_pos, interactable, is_active
	) in &interactable_query {
		let pos = interactable_pos.translation();
		let x = pos.x - player_pos.x;
		let y = pos.y - player_pos.y;
		
		if (x * x) + (y * y) > INTERACT_RADIUS_2 {
			if is_active.is_some() {
				commands.entity(entity).remove::<ActiveInteractable>();
			}
			
			continue;
		}
		
		if let Some(coords) = main_camera.0.world_to_viewport(main_camera.1, pos) {
			*label.0 = Visibility::Visible;
			label.1.position = UiRect {
				left: Val::Px(coords.x + interactable.label_offset.x),
				bottom: Val::Px(coords.y + interactable.label_offset.y),
				..default()
			};
			
			commands.entity(entity).insert(ActiveInteractable);
			
			return;
		}
	}
	
	*label.0 = Visibility::Hidden;
}

fn handle_interact (
	mut query : Query<&mut Interactable, With<ActiveInteractable>>,
	input : Res<Input<KeyCode>>,
) {
	let Some(mut interactable) = query.get_single_mut().ok() else { return };
	
	if input.just_pressed(KeyCode::E) {
		interactable.state = InteractableState::Active;
	} else {
		interactable.state = InteractableState::Waiting;
	}
}
