extern crate indicatif;
extern crate rand;

use std::fs::File;
use std::io::prelude::*;

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

fn main() -> std::io::Result<()> {
    let run_data = run_simulation(
        100_000,
        24,
        Photon::new(Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        &BOUNDS_MIN,
        &BOUNDS_MAX,
    );
    let mut file = File::create("run_data.txt")?;
    file.write_all(build_vec_string(run_data).as_bytes())?;
    Ok(())
}

fn build_vec_string(data: Vec<Vec<Vec3>>) -> String {
    let mut string = String::new();
    for photon in data {
        for point in photon {
            string.push_str(&format!("{} {} {} ", point.x, point.y, point.z));
        }
        string.push('\n');
    }
    string
}
