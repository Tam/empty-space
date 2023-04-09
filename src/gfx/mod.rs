use bevy::app::App;
use bevy::prelude::Plugin;
use crate::gfx::materials::MaterialsPlugin;
use crate::gfx::post_process::PostProcessPlugin;

mod post_process;
mod materials;

pub use materials::*;

pub struct GfxPlugin;
impl Plugin for GfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(PostProcessPlugin)
			.add_plugin(MaterialsPlugin)
		;
	}
}
