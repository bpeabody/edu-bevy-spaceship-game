use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    bundles::MovingObjectBundle,
    components::{Acceleration, Velocity},
};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;

const VELOCITY_MAGNITUDE: f32 = 5.0; // aka speed
const ACCELERATION_MAGNITUDE: f32 = 1.0;

const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer(Timer);

fn rand_xy<R: Rng>(rng: &mut R, xs: Range<f32>, zs: Range<f32>) -> Vec3 {
    Vec3::new(rng.gen_range(xs), 0.0, rng.gen_range(zs))
}

fn rand_dir<R: Rng>(rng: &mut R) -> Vec3 {
    rand_xy(rng, -1.0..1.0, -1.0..1.0).normalize_or_zero()
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let rng = &mut rand::thread_rng();
    let translation = rand_xy(rng, SPAWN_RANGE_X, SPAWN_RANGE_Z);
    let velocity = rand_dir(rng) * VELOCITY_MAGNITUDE;
    let acceleration = rand_dir(rng) * ACCELERATION_MAGNITUDE;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(velocity),
            acceleration: Acceleration(acceleration),
            model: SceneBundle {
                scene: asset_server.load("Rock.glb#Scene0"),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_TIME_SECONDS,
            TimerMode::Repeating,
        )))
        .add_systems(Update, spawn_asteroid);
    }
}
