use bevy::prelude::*;
use crate::{core_setup, Movement, Tilesheet};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerThruster;

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
		Player,
		Movement {
			move_speed: 500.,
			turn_speed: 360f32.to_radians(),
			move_decay: 0.9,
			turn_decay: 0.9,
			..default()
		},
	)).with_children(|commands| {
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.0.clone(),
				sprite: TextureAtlasSprite {
					index: 47,
					color: Color::WHITE.with_a(0.75),
					..default()
				},
				visibility: Visibility::Hidden,
				transform: Transform::from_scale(Vec3::new(0.4, 0.5, 1.))
					.with_translation(Vec3::new(0., -50., 0.)),
				..default()
			},
			PlayerThruster,
		));
	});
}

fn player_movement(
	mut query: Query<&mut Movement, With<Player>>,
	mut thruster_query : Query<&mut Visibility, With<PlayerThruster>>,
	input: Res<Input<KeyCode>>,
) {
	if query.is_empty() { return; }
	let mut movement = query.single_mut();
	
	// Yaw
	if input.pressed(KeyCode::A) { movement.turn_factor =  1.; }
	if input.pressed(KeyCode::D) { movement.turn_factor = -1.; }
	
	// Thrust
	if input.pressed(KeyCode::W) {
		movement.move_factor =  1.;
		*thruster_query.single_mut() = Visibility::Visible;
	} else {
		*thruster_query.single_mut() = Visibility::Hidden;
	}
}
