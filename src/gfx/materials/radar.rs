use bevy::prelude::Color;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a19fa1f5-d3c8-4e4c-aaf6-cedac0a11d28"]
pub struct RadarMaterial {
	#[uniform(0)]
	pub tint: Color,
}

impl Material2d for RadarMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/radar.wgsl".into()
	}
}
