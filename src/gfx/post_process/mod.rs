mod post_process_pipeline;
mod post_process_node;

use bevy::app::App;
use bevy::core_pipeline::core_2d;
use bevy::prelude::Plugin;
use bevy::render::render_graph::RenderGraph;
use bevy::render::RenderApp;
use crate::gfx::post_process::post_process_node::PostProcessNode;
use crate::gfx::post_process::post_process_pipeline::PostProcessPipeline;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
	fn build(&self, app: &mut App) {
		// We need to get the render app from the main app
		let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
			return;
		};
		
		// Initialise the pipeline
		render_app.init_resource::<PostProcessPipeline>();
		
		// Create the node with the render world
		let node = PostProcessNode::new(&mut render_app.world);
		
		// Get the render graph for the entire app
		let mut graph = render_app.world.resource_mut::<RenderGraph>();
		
		// Get the render graph for 2D cameras/views
		let core_2d_graph = graph.get_sub_graph_mut(core_2d::graph::NAME).unwrap();
		
		// Register the post process node in the 2D render graph
		core_2d_graph.add_node(PostProcessNode::NAME, node);
		
		// A slot edge tells the render graph which input/output value should be passed to the node.
		// In this case, the view entity, which is the entity associated with the camera on which the graph is running.
		core_2d_graph.add_slot_edge(
			core_2d_graph.input_node().id,
			core_2d::graph::input::VIEW_ENTITY,
			PostProcessNode::NAME,
			PostProcessNode::IN_VIEW,
		);
		
		// Add an edge between our node and the nodes from bevy to ensure it's ordered correctly
		// Have the node run after tonemapping but before the end of the main pass
		core_2d_graph.add_node_edge(core_2d::graph::node::TONEMAPPING, PostProcessNode::NAME);
		core_2d_graph.add_node_edge(PostProcessNode::NAME, core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING);
	}
}
