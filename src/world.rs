use rand::distributions::{UnitSphereSurface, Distribution};

use crate::photon::{Photon, Vec3};

pub struct PhotonRun {
    photon: Photon,
    scattering_positions: Vec<Vec3>,
}

// returns (theta, phi)
pub fn random_direction(dir: &Vec3) -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let v = Vec3::from(sphere.sample(&mut rand::thread_rng()));
    v
}
