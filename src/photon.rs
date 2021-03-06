use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Photon {
    pub position: Vec3,
    pub direction: Vec3,
}

impl Photon {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Photon {
            position: pos,
            direction: dir,
        }
    }

    pub fn step(&mut self, dx: f32) {
        self.position += &self.direction * dx;
    }
}

