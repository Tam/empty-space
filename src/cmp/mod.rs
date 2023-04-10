use bevy::app::App;
use bevy::prelude::Plugin;

pub use movement::Movement;
pub use parallax::Parallax;
pub use spin::Spin;
pub use tracker::{Tracker, TrackerType};
pub use interactable::Interactable;

mod movement;
mod parallax;
mod spin;
mod tracker;
mod interactable;

pub struct CmpPlugin;
impl Plugin for CmpPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(movement::movement)
			.add_system(parallax::parallax)
			.add_system(spin::spin)
		;
	}
}
