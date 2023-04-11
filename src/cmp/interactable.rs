use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Default, PartialEq)]
pub enum InteractableState {
	#[default]
	Waiting,
	Active,
}

#[derive(Component, Default)]
pub struct Interactable {
	pub label_offset : Vec2,
	pub state : InteractableState,
}

#[derive(Component)]
pub struct ActiveInteractable;
