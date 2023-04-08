mod gfx;
mod player;
mod camera;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, PresentMode};
use crate::camera::CameraPlugin;
use crate::gfx::{GfxPlugin, Parallax};
use crate::player::PlayerPlugin;

#[derive(Resource, Default)]
pub struct Tilesheet (Handle<TextureAtlas>);

fn main() {
	let mut app = App::new();
	
	app
		.init_resource::<Tilesheet>()
		.insert_resource(ClearColor(Color::NONE))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Empty Space".into(),
				resolution: (720., 720.).into(),
				canvas: Some("#canvas".into()),
				resizable: false,
				#[cfg(target_os = "macos")]
				composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
				decorations: false,
				transparent: true,
				present_mode: PresentMode::AutoVsync,
				..default()
			}),
			..default()
		}).set(AssetPlugin {
			#[cfg(feature = "debug_watch")]
			watch_for_changes: true,
			..default()
		}))
		.add_plugin(GfxPlugin)
		.add_plugin(CameraPlugin)
		.add_plugin(PlayerPlugin)
		.add_startup_system(core_setup)
	;
	
	#[cfg(not(target_arch = "wasm32"))]
	app.add_system(window_move);
	
	app.run();
}

pub fn core_setup(
	mut commands : Commands,
	assets_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut tilesheet: ResMut<Tilesheet>,
) {
	// Load texture
	let texture_handle = assets_server.load("tilesheet.png");
	let texture_atlas = TextureAtlas::from_grid(
		texture_handle,
		Vec2::splat(128.),
		8,
		6,
		None,
		None,
	);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);
	tilesheet.0 = texture_atlas_handle.clone();
	
	// TODO: completely tinted, sharpened (?), sprite shader
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: tilesheet.0.clone(),
			sprite: TextureAtlasSprite {
				index: 23,
				color: Color::hex("#232427").unwrap(),
				..default()
			},
			transform: Transform::from_scale(Vec3::splat(10.)),
			..default()
		},
		Parallax(-2),
	));
	
	// TODO: Add sprite blur
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: tilesheet.0.clone(),
			sprite: TextureAtlasSprite {
				index: 25,
				color: Color::hex("#74748c").unwrap().with_a(0.5),
				..default()
			},
			transform: Transform::from_scale(Vec3::splat(2.))
				.with_translation(Vec3::new(0., 0., 2.)),
			..default()
		},
		Parallax(2),
	));
	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: tilesheet.0.clone(),
			sprite: TextureAtlasSprite {
				index: 26,
				color: Color::hex("#74748c").unwrap().with_a(0.5),
				..default()
			},
			transform: Transform::from_scale(Vec3::splat(1.))
				.with_translation(Vec3::new(100., 100., 2.)),
			..default()
		},
		Parallax(1),
	));
}

#[cfg(not(target_arch = "wasm32"))]
fn window_move(
	mouse_button_input: Res<Input<MouseButton>>,
	mut mouse_move_input: EventReader<MouseMotion>,
	mut window_moved: EventReader<WindowMoved>,
	mut windows: Query<&mut Window>,
	mut last_window_position: Local<IVec2>,
) {
	if last_window_position.x == 0 && last_window_position.y == 0 {
		let mut window = windows.single_mut();
		window.position.center(MonitorSelection::Current);
	}
	
	if let Some(moved) = window_moved.iter().next() {
		*last_window_position = moved.position;
	}
	
	if !mouse_button_input.pressed(MouseButton::Left) { return; }
	if let Some(motion) = mouse_move_input.iter().next() {
		let mut window = windows.single_mut();
		let scale = window.resolution.scale_factor() as i32;
		window.position.set(
			*last_window_position + IVec2::from([motion.delta.x as i32, motion.delta.y as i32]) * scale
		);
	}
}
