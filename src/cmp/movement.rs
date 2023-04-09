use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movement {
	pub move_speed  : f32,
	pub turn_speed  : f32,
	pub move_factor : f32,
	pub turn_factor : f32,
	pub move_decay  : f32,
	pub turn_decay  : f32,
}

pub fn movement (
	mut query : Query<(&mut Movement, &mut Transform)>,
	time : Res<Time>,
) {
	for (mut movement, mut transform) in &mut query {
		transform.rotate_z(movement.turn_factor * movement.turn_speed * time.delta_seconds());
		
		let dir = transform.rotation * Vec3::Y;
		let dist = movement.move_factor * movement.move_speed * time.delta_seconds();
		transform.translation += dir * dist;
		
		movement.move_factor *= movement.move_decay;
		movement.turn_factor *= movement.turn_decay;
	}
}
