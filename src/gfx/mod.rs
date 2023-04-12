use bevy::app::App;
use bevy::prelude::Plugin;
use crate::gfx::materials::MaterialsPlugin;
use crate::gfx::post_process::PostProcessPlugin;

mod post_process;
mod materials;
mod shader_imports;

pub use materials::*;
use crate::gfx::shader_imports::ShaderImports;

pub struct GfxPlugin;
impl Plugin for GfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<ShaderImports>()
			.add_plugin(PostProcessPlugin)
			.add_plugin(MaterialsPlugin)
		;
	}
}
