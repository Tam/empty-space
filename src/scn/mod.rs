mod game;

use bevy::prelude::*;
use crate::scn::game::GameScenePlugin;

pub struct ScnPlugin;
impl Plugin for ScnPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(GameScenePlugin)
		;
	}
}
