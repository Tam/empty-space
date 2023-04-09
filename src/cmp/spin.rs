use bevy::prelude::*;

#[derive(Component)]
pub struct Spin (pub f32);

pub fn spin (
	mut query : Query<(&mut Transform, &Spin)>,
	time : Res<Time>,
) {
	for (mut transform, spin) in &mut query {
		transform.rotate_z(spin.0.to_radians() * time.delta_seconds());
	}
}
