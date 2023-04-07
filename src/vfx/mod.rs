use bevy::app::App;
use bevy::prelude::Plugin;
use crate::vfx::post_process::PostProcessPlugin;

mod post_process;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(PostProcessPlugin)
		;
	}
}
