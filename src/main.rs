extern crate indicatif;
extern crate rand;
#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

pub mod photon;
pub mod simulation;
pub mod vec3;
pub mod world;

use crate::photon::Photon;
use crate::simulation::run_simulation;
use crate::vec3::Vec3;

const BOUNDS_MIN: (f32, f32, f32) = (0.0, 0.0, 0.0);
const BOUNDS_MAX: (f32, f32, f32) = (1.0, 1.0, 1.0);

const PHOTON_POSITION: (f32, f32, f32) = (0.5, 0.5, 0.0);
const PHOTON_DIRECTION: (f32, f32, f32) = (0.0, 0.0, 1.0);

const PHOTON_NUM_DEFAULT: usize = 1000;
const CORE_NUM_DEFAULT: usize = 1;

const PARTICLE_RADIUS: f32 = 0.00001;

const STEP_SIZE: f32 = 0.000_01;

fn main() -> std::io::Result<()> {
    let matches = App::new("Photon Tracer")
        .author("Max Tyler <maxastyler@gmail.com>")
        .about("Trace the path of a photon through air with arbitrary density")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .help("Sets the file to save the photon paths to")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("num_photons")
                .short("p")
                .help(&format!(
                    "The number of photon paths to simulate. Default is {}",
                    PHOTON_NUM_DEFAULT
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_cores")
                .short("c")
                .help(&format!(
                    "The number of CPU cores to use. Default is {}",
                    CORE_NUM_DEFAULT
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("min_bounds")
                .short("m")
                .help(&format!(
                    "The minimum point of the bounding box. Default is {} {} {}",
                    BOUNDS_MIN.0, BOUNDS_MIN.1, BOUNDS_MIN.2
                ))
                .takes_value(true)
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("max_bounds")
                .short("M")
                .help(&format!(
                    "The maximum point of the bounding box. Default is {} {} {}",
                    BOUNDS_MAX.0, BOUNDS_MAX.1, BOUNDS_MAX.2
                ))
                .takes_value(true)
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("photon_pos")
                .short("P")
                .help(&format!(
                    "The starting position of the photon. Default is {} {} {}",
                    PHOTON_POSITION.0, PHOTON_POSITION.1, PHOTON_POSITION.2
                ))
                .takes_value(true)
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("photon_dir")
                .short("d")
                .help(&format!(
                    "The starting direction of the photon. MAKE SURE THIS IS NORMALISED TO 1. Default is {} {} {}",
                    PHOTON_DIRECTION.0, PHOTON_DIRECTION.1, PHOTON_DIRECTION.2
                ))
                .takes_value(true)
                .number_of_values(3),
        )
        .arg(
            Arg::with_name("particle_radius")
                .short("r")
                .help(&format!(
                    "The radius of the particles. Default is {}",
                    PARTICLE_RADIUS
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("step_size")
                .short("x")
                .help(&format!(
                    "The step distance of the photon. Default is {}",
                    STEP_SIZE
                ))
                .takes_value(true)
        )
        .get_matches();
    let b_min_vec = values_t!(matches.values_of("min_bounds"), f32).unwrap_or(vec![
        BOUNDS_MIN.0,
        BOUNDS_MIN.1,
        BOUNDS_MIN.2,
    ]);
    let b_max_vec = values_t!(matches.values_of("max_bounds"), f32).unwrap_or(vec![
        BOUNDS_MAX.0,
        BOUNDS_MAX.1,
        BOUNDS_MAX.2,
    ]);

    let bounds_min = Vec3::new(b_min_vec[0], b_min_vec[1], b_min_vec[2]);
    let bounds_max = Vec3::new(b_max_vec[0], b_max_vec[1], b_max_vec[2]);

    let p_pos_vec = values_t!(matches.values_of("photon_pos"), f32).unwrap_or(vec![
        PHOTON_POSITION.0,
        PHOTON_POSITION.1,
        PHOTON_POSITION.2,
    ]);
    let p_dir_vec = values_t!(matches.values_of("photon_dir"), f32).unwrap_or(vec![
        PHOTON_DIRECTION.0,
        PHOTON_DIRECTION.1,
        PHOTON_DIRECTION.2,
    ]);

    let p_pos_vec = Vec3::new(p_pos_vec[0], p_pos_vec[1], p_pos_vec[2]);
    let p_dir_vec = Vec3::new(p_dir_vec[0], p_dir_vec[1], p_dir_vec[2]);

    let run_data = run_simulation(
        value_t!(matches, "num_photons", usize).unwrap_or(PHOTON_NUM_DEFAULT),
        value_t!(matches, "num_cores", usize).unwrap_or(CORE_NUM_DEFAULT),
        Photon::new(p_pos_vec, p_dir_vec),
        bounds_min,
        bounds_max,
        value_t!(matches, "particle_radius", f32)
            .unwrap_or(PARTICLE_RADIUS)
            .powi(2)
            * std::f32::consts::PI,
        value_t!(matches, "step_size", f32).unwrap_or(STEP_SIZE),
    );
    let mut file = File::create(matches.value_of("OUTPUT").unwrap())?;
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
