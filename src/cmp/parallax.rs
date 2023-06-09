use bevy::prelude::*;
use crate::MainCamera;

#[derive(Component)]
pub struct Parallax (pub isize);

const LAYER_MOVE_SPEED_SLOW : f32 = 0.1;
const LAYER_MOVE_SPEED_FAST : f32 = -0.5;

pub fn parallax (
	camera_query : Query<&Transform, (With<MainCamera>, Without<Parallax>)>,
	mut entity_query : Query<(&mut Transform, &Parallax), Without<MainCamera>>,
	mut prev_camera_t : Local<Vec2>,
) {
	let camera_t = camera_query.single().translation.truncate();
	let change = camera_t - *prev_camera_t;
	
	for (mut t, depth) in &mut entity_query {
		if depth.0 == 0 { continue; }
		let f_depth = depth.0 as f32;
		
		if depth.0 < 0 {
			t.translation += (change * LAYER_MOVE_SPEED_SLOW * f32::abs(f_depth)).extend(0.);
		} else {
			t.translation += (change * LAYER_MOVE_SPEED_FAST * f_depth).extend(0.);
		}
	}
	
	*prev_camera_t = camera_t;
}
