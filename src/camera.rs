use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup)
			.add_system(follow_player)
		;
	}
}

fn setup (
	mut commands : Commands,
) {
	commands.spawn(Camera2dBundle::default());
}

fn follow_player (
	player_query : Query<&Transform, (With<Player>, Without<Camera2d>)>,
	mut camera_query : Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
	time : Res<Time>,
) {
	if player_query.is_empty() { return; }
	let player_t = player_query.single();
	let mut camera_t = camera_query.single_mut();
	
	camera_t.translation = Vec3::lerp(
		camera_t.translation,
		player_t.translation,
		time.delta_seconds() * 10.,
	);
}
