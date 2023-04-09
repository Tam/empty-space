mod radar;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
pub use crate::gfx::materials::radar::RadarMaterial;

pub struct MaterialsPlugin;
impl Plugin for MaterialsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(Material2dPlugin::<RadarMaterial>::default())
		;
	}
}
