use bevy::app::App;
use bevy::prelude::Plugin;
pub use parallax::Parallax;
pub use spin::Spin;
use crate::gfx::parallax::parallax_offset;
use crate::gfx::spin::spin;
use crate::gfx::post_process::PostProcessPlugin;

mod post_process;
mod parallax;
mod spin;


pub struct GfxPlugin;
impl Plugin for GfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(PostProcessPlugin)
			.add_system(parallax_offset)
			.add_system(spin)
		;
	}
}
