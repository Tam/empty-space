use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use crate::AppState;
use crate::gfx::RadarMaterial;
use crate::scn::game::GameState;

#[derive(Component)]
struct RadarRoot;

#[derive(Component)]
struct RadarUIRoot;

pub struct RadarPlugin;
impl Plugin for RadarPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(setup.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown.in_schedule(OnExit(AppState::Game)))
			.add_system(show.in_schedule(OnEnter(GameState::Radar)))
			.add_system(hide.in_schedule(OnExit(GameState::Radar)))
		;
	}
}

fn setup (
	mut commands : Commands,
	mut images: ResMut<Assets<Image>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<RadarMaterial>>,
) {
	// Radar renderer
	let size = Extent3d {
		height: 1440,
		width: 1440,
		..default()
	};
	
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::Bgra8UnormSrgb,
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		},
		..default()
	};
	image.resize(size);
	let image_handle = images.add(image);
	
	let radar_render_layer = RenderLayers::layer(1);
	
	commands.spawn((
		RadarRoot,
		SpatialBundle::default(),
	)).with_children(|commands| {
		commands.spawn((
			MaterialMesh2dBundle {
				mesh: meshes.add(shape::Quad::new(Vec2::splat(1440.)).into()).into(),
				material: materials.add(RadarMaterial {}),
				..default()
			},
			radar_render_layer,
		));
		
		commands.spawn((
			Camera2dBundle {
				camera_2d: Camera2d { clear_color: ClearColorConfig::None },
				camera: Camera {
					order: -1,
					target: RenderTarget::Image(image_handle.clone()),
					..default()
				},
				..default()
			},
			UiCameraConfig {
				show_ui: false,
			},
			radar_render_layer,
		));
	});
	
	// UI
	commands.spawn((
		RadarUIRoot,
		ImageBundle {
			style: Style {
				size: Size::all(Val::Percent(100.)),
				..default()
			},
			image: image_handle.clone().into(),
			visibility: Visibility::Hidden,
			..default()
		},
	));
}

fn teardown (
	mut commands : Commands,
	query: Query<Entity, Or<(With<RadarUIRoot>, With<RadarRoot>)>>,
) {
	for e in &query {
		commands.entity(e).despawn_recursive();
	}
}

fn show (
	mut query: Query<&mut Visibility, With<RadarUIRoot>>,
) {
	*query.single_mut() = Visibility::Visible;
}

fn hide (
	mut query: Query<&mut Visibility, With<RadarUIRoot>>,
) {
	*query.single_mut() = Visibility::Hidden;
}
