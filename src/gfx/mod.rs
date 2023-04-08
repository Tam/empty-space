use bevy::app::App;
use bevy::prelude::Plugin;
use crate::gfx::parallax::ParallaxPlugin;
use crate::gfx::post_process::PostProcessPlugin;

mod post_process;
mod parallax;

pub struct GfxPlugin;
pub use parallax::Parallax;

impl Plugin for GfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(PostProcessPlugin)
			.add_plugin(ParallaxPlugin)
		;
	}
}
