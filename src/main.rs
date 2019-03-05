extern crate rayon;
extern crate rand;

use rayon::prelude::*;

pub mod photon;
pub mod world;

use crate::photon::{Photon, Vec3};
use crate::world::random_direction;

fn move_photon(p: &mut Photon, dt: f32) {
    if (p.position.x > 0.0)
        && (p.position.x < 1.0)
        && (p.position.y > 0.0)
        && (p.position.y < 1.0)
        && (p.position.z > 0.0)
        && (p.position.z < 1.0)
    {
    }
}

fn main() {
    println!("{:?}", random_direction(&Vec3::zero()));
}
