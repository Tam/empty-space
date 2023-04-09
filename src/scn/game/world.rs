use bevy::prelude::*;
use crate::cmp::{Parallax, Spin};

pub fn setup (
	commands : &mut ChildBuilder,
	tilesheet : Handle<TextureAtlas>,
) {
	// TODO: sharpened (?), sprite shader
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: tilesheet.clone(),
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
			texture_atlas: tilesheet.clone(),
			sprite: TextureAtlasSprite {
				index: 33,
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
			texture_atlas: tilesheet.clone(),
			sprite: TextureAtlasSprite {
				index: 34,
				color: Color::hex("#74748c").unwrap().with_a(0.5),
				..default()
			},
			transform: Transform::from_scale(Vec3::splat(1.))
				.with_translation(Vec3::new(100., 100., 2.)),
			..default()
		},
		Parallax(1),
	));
}
