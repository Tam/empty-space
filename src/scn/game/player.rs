use bevy::prelude::*;
use crate::cmp::Movement;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerThruster;

pub fn setup(
	commands : &mut ChildBuilder,
	tilesheet : Handle<TextureAtlas>,
) {
	commands.spawn((
		Player,
		Movement {
			move_speed: 500.,
			turn_speed: 270f32.to_radians(),
			move_decay: 0.9,
			turn_decay: 0.9,
			..default()
		},
		SpatialBundle {
			transform: Transform::from_xyz(0., 0., 1.),
			..default()
		},
	)).with_children(|commands| {
		// Thruster
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.clone(),
				sprite: TextureAtlasSprite {
					index: 47,
					color: Color::WHITE.with_a(0.75),
					..default()
				},
				visibility: Visibility::Hidden,
				transform: Transform::from_scale(Vec3::new(0.2, 0.25, 1.))
					.with_translation(Vec3::new(0., -10., 0.)),
				..default()
			},
			PlayerThruster,
		));
		
		// Ship
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.clone(),
				sprite: TextureAtlasSprite::new(1),
				transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.))
					.with_translation(Vec3::new(0., 8., 1.)),
				..default()
			},
		));
	});
}

// TODO: extract thruster visibility to own generic system
pub fn movement(
	mut query: Query<&mut Movement, With<Player>>,
	mut thruster_query : Query<&mut Visibility, With<PlayerThruster>>,
	input: Res<Input<KeyCode>>,
) {
	if query.is_empty() { return; }
	let mut movement = query.single_mut();
	
	// Yaw
	if input.pressed(KeyCode::A) { movement.turn_factor += 0.1; }
	if input.pressed(KeyCode::D) { movement.turn_factor -= 0.1; }
	movement.turn_factor = movement.turn_factor.clamp(-1., 1.);
	
	// Thrust
	if input.pressed(KeyCode::W) {
		movement.move_factor += 0.1;
		movement.move_factor = movement.move_factor.clamp(0., 1.);
		*thruster_query.single_mut() = Visibility::Visible;
	} else {
		*thruster_query.single_mut() = Visibility::Hidden;
	}
}
