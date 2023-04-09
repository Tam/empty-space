use bevy::app::App;
use bevy::prelude::Plugin;

pub use movement::Movement;
pub use parallax::Parallax;
pub use spin::Spin;

use crate::cmp::movement::movement;
use crate::cmp::parallax::parallax;
use crate::cmp::spin::spin;

mod movement;
mod parallax;
mod spin;

pub struct CmpPlugin;
impl Plugin for CmpPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(movement)
			.add_system(parallax)
			.add_system(spin)
		;
	}
}
