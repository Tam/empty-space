use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Interactable {
	pub label_offset : Vec2,
}
