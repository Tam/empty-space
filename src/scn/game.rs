use bevy::prelude::*;
use crate::{AppState, Tilesheet};
use crate::cmp::{Parallax, Spin};
use crate::player::{player_movement, setup_player};

#[derive(Component)]
struct GameSceneRoot;

pub struct GameScenePlugin;
impl Plugin for GameScenePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(setup.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown.in_schedule(OnExit(AppState::Game)))
			.add_system(player_movement.in_set(OnUpdate(AppState::Game)))
		;
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
		// Map
		// -------------------------------------------------------------------------
		
		// TODO: completely tinted, sharpened (?), sprite shader
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.0.clone(),
				sprite: TextureAtlasSprite {
					index: 23,
					color: Color::hex("#232427").unwrap(),
					..default()
				},
				transform: Transform::from_scale(Vec3::new(10., 10., -2.)),
				..default()
			},
			Parallax(-2),
			Spin(2.),
		));
		
		// TODO: Add sprite blur
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.0.clone(),
				sprite: TextureAtlasSprite {
					index: 25,
					color: Color::hex("#74748c").unwrap().with_a(0.5),
					..default()
				},
				transform: Transform::from_scale(Vec3::splat(2.))
					.with_translation(Vec3::new(0., 0., 2.)),
				..default()
			},
			Parallax(2),
		));
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: tilesheet.0.clone(),
				sprite: TextureAtlasSprite {
					index: 26,
					color: Color::hex("#74748c").unwrap().with_a(0.5),
					..default()
				},
				transform: Transform::from_scale(Vec3::splat(1.))
					.with_translation(Vec3::new(100., 100., 2.)),
				..default()
			},
			Parallax(1),
		));
		
		// Player
		// -------------------------------------------------------------------------
		
		setup_player(commands, tilesheet);
	});
}

fn teardown (
	mut commands : Commands,
	query: Query<Entity, With<GameSceneRoot>>,
) {
	commands.entity(query.single()).despawn_recursive();
}
