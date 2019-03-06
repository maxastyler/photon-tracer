extern crate rand;
extern crate indicatif;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub mod photon;
pub mod simulation;
pub mod vec3;
pub mod world;

use crate::photon::Photon;
use crate::simulation::run_simulation;
use crate::vec3::Vec3;

const SPEED_OF_LIGHT: f32 = 299_792_458.0;
const BOUNDS_MIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const BOUNDS_MAX: Vec3 = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

fn main() {
    for i in run_simulation(
        100_000,
        4,
        Photon::new(Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        &BOUNDS_MIN,
        &BOUNDS_MAX,
    ) {
        println!("{:?}", i);
    }
}
