use bevy::prelude::Component;

pub enum TrackerType {
	Resource,
	Enemy,
}

#[derive(Component)]
pub struct Tracker (pub TrackerType);
