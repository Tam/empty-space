use bevy::prelude::*;
use crate::{core_setup, Tilesheet};

#[derive(Component, Default)]
pub struct Player {
	move_speed : f32,
	turn_speed : f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system(setup.after(core_setup))
			.add_system(player_movement)
		;
	}
}

fn setup (
	mut commands : Commands,
	tilesheet : Res<Tilesheet>,
) {
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: tilesheet.0.clone(),
			sprite: TextureAtlasSprite::new(1),
			transform: Transform::from_scale(Vec3::splat(0.5)),
			..default()
		},
		Player {
			move_speed: 500.,
			turn_speed: 360f32.to_radians(),
		},
	));
}

fn player_movement(
	mut query: Query<(&Player, &mut Transform)>,
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	mut rotation_factor : Local<f32>,
	mut movement_factor : Local<f32>,
) {
	if query.is_empty() { return; }
	let (player, mut t) = query.single_mut();
	
	// Yaw
	if input.pressed(KeyCode::A) { *rotation_factor =  1.; }
	if input.pressed(KeyCode::D) { *rotation_factor = -1.; }
	
	// Thrust
	if input.pressed(KeyCode::W) { *movement_factor = 1.; }
	
	// Transform
	t.rotate_z(*rotation_factor * player.turn_speed * time.delta_seconds());
	let move_dir = t.rotation * Vec3::Y;
	let move_dist = *movement_factor * player.move_speed * time.delta_seconds();
	t.translation += move_dir * move_dist;
	
	// Decay
	*rotation_factor *= 0.9;
	*movement_factor *= 0.7;
}
