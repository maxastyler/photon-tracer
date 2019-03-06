use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn mag_sq(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Vec3 {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl From<&[f32; 3]> for Vec3 {
    fn from(a: &[f32; 3]) -> Self {
        Vec3 {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(a: [f64; 3]) -> Self {
        Vec3 {
            x: a[0] as f32,
            y: a[1] as f32,
            z: a[2] as f32,
        }
    }
}

impl From<&[f64; 3]> for Vec3 {
    fn from(a: &[f64; 3]) -> Self {
        Vec3 {
            x: a[0] as f32,
            y: a[1] as f32,
            z: a[2] as f32,
        }
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'a Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a> SubAssign<&'a Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &'a Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn add_works() {
        let a = Vec3::new(1.0, 2.0, -2.0);
        let b = Vec3::new(-2.0, 3.0, -4.0);
        assert_eq!(a + b, Vec3::new(-1.0, 5.0, -6.0));
    }

    #[test]
    fn assign_add_works() {
        let mut a = Vec3::new(1.0, 2.0, -2.0);
        a += Vec3::new(-2.0, 3.0, -4.0);
        assert_eq!(a, Vec3::new(-1.0, 5.0, -6.0));
    }

    #[test]
    fn sub_works() {
        let a = Vec3::new(1.0, 2.0, -2.0);
        let b = Vec3::new(-2.0, 3.0, -4.0);
        assert_eq!(a - b, Vec3::new(3.0, -1.0, 2.0));
    }

    #[test]
    fn assign_sub_works() {
        let mut a = Vec3::new(1.0, 2.0, -2.0);
        a -= Vec3::new(-2.0, 3.0, -4.0);
        assert_eq!(a, Vec3::new(3.0, -1.0, 2.0));
    }

    #[test]
    fn mul_works() {
        let a = Vec3::new(1.0, 2.0, -2.0);
        assert_eq!(a.clone() * 2.0, Vec3::new(2.0, 4.0, -4.0));
        assert_eq!(2.0 * a, Vec3::new(2.0, 4.0, -4.0));
    }

    #[test]
    fn mul_assign_works() {
        let mut a = Vec3::new(1.0, 2.0, -2.0);
        a *= 2.0;
        assert_eq!(a, Vec3::new(2.0, 4.0, -4.0));
    }
}
