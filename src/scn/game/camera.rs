use bevy::prelude::*;
use crate::MainCamera;
use crate::scn::game::player::Player;

pub fn follow_player (
	player_query : Query<&Transform, (With<Player>, Without<MainCamera>)>,
	mut camera_query : Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
	time : Res<Time>,
) {
	if player_query.is_empty() { return; }
	let player_t = player_query.single();
	let mut camera_t = camera_query.single_mut();
	
	camera_t.translation = Vec2::lerp(
		camera_t.translation.truncate(),
		player_t.translation.truncate(),
		time.delta_seconds() * 10.,
	).extend(camera_t.translation.z);
}
