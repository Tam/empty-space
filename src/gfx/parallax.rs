use bevy::prelude::*;

#[derive(Component)]
pub struct Parallax (pub isize);

const LAYER_MOVE_SPEED_SLOW : f32 = 0.1;
const LAYER_MOVE_SPEED_FAST : f32 = -0.5;

pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(parallax_offset)
		;
	}
}

fn parallax_offset (
	camera_query : Query<&Transform, (With<Camera2d>, Without<Parallax>)>,
	mut entity_query : Query<(&mut Transform, &Parallax), Without<Camera2d>>,
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
