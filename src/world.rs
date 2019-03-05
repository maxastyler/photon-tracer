use rand::distributions::{Distribution, UnitSphereSurface};
use rand::{Rng, ThreadRng};

use crate::photon::Photon;
use crate::vec3::Vec3;

pub struct PhotonRun<'a> {
    pub photon: Photon,
    pub scattering_positions: Vec<Vec3>,
    pub density: &'a Fn(Vec3) -> f32,
    pub bounds: (&'a Vec3, &'a Vec3),
    pub particle_volume: f32,
}

impl<'a> PhotonRun<'a> {
    pub fn new(
        photon: Photon,
        density: &'a Fn(Vec3) -> f32,
        bounds: (&'a Vec3, &'a Vec3),
        particle_volume: f32,
    ) -> PhotonRun<'a> {
        PhotonRun {
            scattering_positions: vec![photon.position.clone()],
            photon: photon,
            density: density,
            bounds: bounds,
            particle_volume: particle_volume,
        }
    }

    pub fn in_box(&self) -> bool {
        if (self.bounds.0.x <= self.photon.position.x)
            && (self.bounds.1.x > self.photon.position.x)
            && (self.bounds.0.y <= self.photon.position.y)
            && (self.bounds.1.y > self.photon.position.y)
            && (self.bounds.0.z <= self.photon.position.z)
            && (self.bounds.1.z > self.photon.position.z)
        {
            true
        } else {
            false
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.photon.step(dt);
        if self.collided() {
            self.photon.direction = random_direction(&self.photon.direction) * 299_792_458.0;
            self.scattering_positions.push(self.photon.position.clone());
        }
    }

    /// work out whether the photon has collided in this last timestep
    pub fn collided(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_bool((self.particle_volume * (self.density)(self.photon.position.clone())).into())
    }
}

// returns (theta, phi)
pub fn random_direction(_dir: &Vec3) -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let v = Vec3::from(sphere.sample(&mut rand::thread_rng()));
    v
}

#[cfg(test)]
mod tests {
    use super::PhotonRun;
    use crate::photon::Photon;
    use crate::vec3::Vec3;

    fn density(_: Vec3) -> f32 {
        1.0
    }

    #[test]
    fn in_box_works() {
        let min_bounds = Vec3::new(0.0, 0.0, 0.0);
        let max_bounds = Vec3::new(1.0, 1.0, 1.0);

        let run = PhotonRun::new(
            Photon::new(Vec3::new(0.5, 0.5, 0.5), Vec3::zero()),
            &density,
            (&min_bounds, &max_bounds),
            1.0,
        );
        assert!(run.in_box());
        let run = PhotonRun::new(
            Photon::new(Vec3::new(1.5, 1.5, 1.5), Vec3::zero()),
            &density,
            (&min_bounds, &max_bounds),
            1.0,
        );
        assert!(!run.in_box());
    }
}
