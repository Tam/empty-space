use bevy::prelude::*;
use crate::{AppState, Tilesheet};
use crate::scn::game::radar::RadarPlugin;

mod player;
mod camera;
mod world;
mod radar;

#[derive(Component)]
struct GameSceneRoot;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum GameState {
	#[default]
	Game,
	Radar,
}

pub struct GameScenePlugin;
impl Plugin for GameScenePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_state::<GameState>()
			.add_plugin(RadarPlugin)
			.add_system(setup.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown.in_schedule(OnExit(AppState::Game)))
			.add_systems((
				player::movement,
			).in_set(OnUpdate(AppState::Game)).in_set(OnUpdate(GameState::Game)))
			.add_systems((
				camera::follow_player,
			).in_set(OnUpdate(AppState::Game)))
		;
		
		#[cfg(feature = "debug")]
		app.add_system(debug_input);
	}
}

fn setup (
	mut commands : Commands,
	tilesheet : Res<Tilesheet>,
) {
	commands.spawn((
		GameSceneRoot,
		SpatialBundle::default(),
	)).with_children(|commands| {
		world::setup(commands, tilesheet.0.clone());
		player::setup(commands, tilesheet.0.clone());
	});
}

fn teardown (
	mut commands : Commands,
	query: Query<Entity, With<GameSceneRoot>>,
) {
	commands.entity(query.single()).despawn_recursive();
}

#[cfg(feature = "debug")]
fn debug_input (
	input: Res<Input<KeyCode>>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	if input.pressed(KeyCode::Space) { next_state.set(GameState::Radar) }
	else if input.just_released(KeyCode::Space) { next_state.set(GameState::Game) }
}
