extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub mod photon;
pub mod vec3;
pub mod world;

use crate::photon::Photon;
use crate::vec3::Vec3;
use crate::world::{random_direction, PhotonRun};

const SPEED_OF_LIGHT: f32 = 299_792_458.0;

fn main() {
    let total_photons = 100;
    let photon_count = Arc::new(AtomicUsize::new(0));
    let mut thread_handles = vec![];
    let (tx, rx) = mpsc::channel();
    for _ in 0..4 {
        let path_clone = photon_count.clone();
        let tx_clone = tx.clone();
        thread_handles.push(thread::spawn(move || {
            let bounds_min = Vec3::new(0.0, 0.0, 0.0);
            let bounds_max = Vec3::new(1.0, 1.0, 1.0);
            loop {
                if path_clone.load(Ordering::SeqCst) < total_photons {
                    path_clone.fetch_add(1, Ordering::SeqCst);
                    let mut r = PhotonRun::new(
                        Photon::new(
                            Vec3::new(0.5, 0.5, 0.0),
                            Vec3::new(0.0, 0.0, 1.0),
                        ),
                        &|_| 1.0,
                        (&bounds_min, &bounds_max),
                        0.01,
                    );
                    while r.in_box() {
                        r.step(0.000_000_000_001);
                    }
                    tx_clone.send(r.scattering_positions).unwrap();
                } else {
                    break;
                }
            }
        }));
    }
    drop(tx);
    for thread in thread_handles {
        thread.join().unwrap();
    }
}
