use std::f32::consts::PI;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use crate::{AppState, Tilesheet};
use crate::cmp::{Tracker, TrackerType};
use crate::gfx::RadarMaterial;
use crate::scn::game::GameState;
use crate::scn::game::player::Player;
use crate::utl::math;

#[derive(Component)]
struct RadarRoot;

#[derive(Component)]
struct RadarUIRoot;

#[derive(Component, Eq, PartialEq, Hash)]
struct TrackerIcon (pub u32);

pub struct RadarPlugin;
impl Plugin for RadarPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(setup.in_schedule(OnEnter(AppState::Game)))
			.add_system(teardown.in_schedule(OnExit(AppState::Game)))
			.add_system(show.in_schedule(OnEnter(GameState::Radar)))
			.add_system(hide.in_schedule(OnExit(GameState::Radar)))
			.add_system(show_tracker_icons.in_set(OnUpdate(GameState::Radar)))
			.add_system(remove_tracker_icons.in_base_set(CoreSet::PostUpdate))
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
				material: materials.add(RadarMaterial {
					tint: Color::hex("#353639").unwrap(),
				}),
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

fn show_tracker_icons (
	mut commands : Commands,
	radar_query: Query<Entity, (With<RadarRoot>, Without<Player>)>,
	tracker_query: Query<(Entity, &Transform, &Tracker), (Without<Player>, Without<TrackerIcon>)>,
	mut icon_query: Query<(&TrackerIcon, &mut Transform, &mut TextureAtlasSprite), Without<Player>>,
	player_query: Query<&Transform, With<Player>>,
	tilesheet : Res<Tilesheet>,
	time : Res<Time>,
) {
	let player_t = player_query.single();
	let radar = radar_query.single();
	let radar_render_layer = RenderLayers::layer(1);
	let mut existing_icons : HashMap<_, _> = icon_query
		.iter_mut()
		.map(|(i, t, s)| (i.0, (t, s)))
		.collect();
	
	for (entity, transform, tracker) in &tracker_query {
		let id = entity.index();
		let t = ((transform.translation - player_t.translation) * 0.3).truncate().extend(1.);
		
		// FIXME: fade is off from shader sweep
		let st = (t / 720.).truncate();
		let mut a = f32::atan2(st.y, st.x);
		a = (a + PI) / (2. * PI);
		a = 1. - math::f_mod(time.elapsed_seconds_wrapped() * 0.5 - a, 1.);
		
		if existing_icons.contains_key(&id) {
			let (icon_t, icon_s) = existing_icons.get_mut(&id).unwrap();
			icon_t.translation = t;
			icon_s.color.set_a(a);
		} else {
			let tint = match tracker.0 {
				TrackerType::Resource => Color::BISQUE,
				TrackerType::Enemy => Color::RED,
			};
			
			commands.spawn((
				TrackerIcon(id),
				SpriteSheetBundle {
					texture_atlas: tilesheet.0.clone(),
					sprite: TextureAtlasSprite {
						index: 28,
						color: tint.with_a(a),
						..default()
					},
					transform: Transform::from_xyz(t.x, t.y, 1.),
					..default()
				},
				radar_render_layer,
			)).set_parent(radar);
		}
	}
}

fn remove_tracker_icons (
	mut commands : Commands,
	mut removed: RemovedComponents<Tracker>,
	query: Query<(Entity, &TrackerIcon)>,
) {
	let icons : HashMap<_, _> = query
		.iter()
		.map(|(e, t)| (t.0, e))
		.collect();
	
	for entity in &mut removed {
		commands.entity(icons[&entity.index()]).despawn_recursive();
	}
}
