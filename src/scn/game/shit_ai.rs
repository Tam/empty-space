use bevy::prelude::{Component, Query, Res, Time, Timer};
use bevy::time::TimerMode;
use rand::prelude::SliceRandom;
use crate::cmp::Movement;

#[derive(Component)]
pub struct ShitAi (Timer);

impl Default for ShitAi {
	fn default() -> Self {
		Self(Timer::from_seconds(2., TimerMode::Repeating))
	}
}

pub fn shit_ai (
	mut query : Query<(&mut Movement, &mut ShitAi)>,
	time : Res<Time>,
) {
	let mut rng = rand::thread_rng();
	let dirs = [-1., 1.];
	
	for (mut m, mut timer) in &mut query {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			m.turn_factor = *dirs.choose(&mut rng).unwrap() as f32;
		}
	}
}
