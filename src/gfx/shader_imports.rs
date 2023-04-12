use bevy::prelude::{AssetServer, FromWorld, Handle, Resource, Shader, World};

#[derive(Resource)]
pub struct ShaderImports (Vec<Handle<Shader>>);

impl FromWorld for ShaderImports {
	fn from_world(world: &mut World) -> Self {
		let assets_server = world.resource::<AssetServer>();
		let mut shaders = Vec::new();
		
		for name in [
			"colour",
			"starfield",
		] {
			shaders.push(assets_server.load(format!("shaders/{name}.wgsl")));
		}
		
		Self(shaders)
	}
}
