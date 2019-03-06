use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

use crate::photon::Photon;
use crate::vec3::Vec3;
use crate::world::PhotonRun;

pub fn density(v: Vec3) -> f32 {
    0.0
}

pub fn run_simulation(
    n_photons: usize,
    n_threads: usize,
    photon_prototype: Photon,
    bounds_min: &'static Vec3,
    bounds_max: &'static Vec3,
    particle_cross_section: f32,
    dx: f32,
) -> Vec<Vec<Vec3>> {
    let photon_count = Arc::new(AtomicUsize::new(0));
    let mut thread_handles = vec![];
    let (tx, rx) = mpsc::channel();
    for _ in 0..n_threads - 1 {
        let photon_count_clone = photon_count.clone();
        let tx_clone = tx.clone();
        let photon_prototype_clone = photon_prototype.clone();
        thread_handles.push(thread::spawn(move || loop {
            if photon_count_clone.load(Ordering::SeqCst) < n_photons {
                photon_count_clone.fetch_add(1, Ordering::SeqCst);
                let mut r = PhotonRun::new(
                    photon_prototype_clone,
                    &density,
                    (bounds_min, bounds_max),
                    particle_cross_section,
                );
                while r.in_box() {
                    r.step(dx);
                }
                r.scattering_positions.push(r.photon.position);
                tx_clone.send(r.scattering_positions).unwrap();
            } else {
                break;
            }
        }));
    }
    let bar = ProgressBar::new(n_photons as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}][{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .progress_chars("##-"),
    );

    let photon_count_clone = photon_count.clone();
    let tx_clone = tx.clone();
    let photon_prototype_clone = photon_prototype.clone();
    loop {
        if photon_count_clone.load(Ordering::SeqCst) < n_photons {
            photon_count_clone.fetch_add(1, Ordering::SeqCst);
            let mut r = PhotonRun::new(
                photon_prototype_clone,
                &density,
                (bounds_min, bounds_max),
                particle_cross_section,
            );
            while r.in_box() {
                r.step(dx);
            }
            bar.set_position(photon_count_clone.load(Ordering::SeqCst) as u64);
            r.scattering_positions.push(r.photon.position);
            tx_clone.send(r.scattering_positions).unwrap();
        } else {
            break;
        }
    }
    bar.finish();
    for handle in thread_handles {
        handle.join().unwrap();
    }
    rx.try_iter().collect()
}
