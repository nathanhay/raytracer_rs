use std::ops;
use std::fmt;
use std::cmp::PartialEq;
use crate::utils::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub enum Vector3 {
    X,
    Y,
    Z,
}

// Type aliases for Vec3
pub use Vec3 as Point3; // 3D point
pub use Vec3 as Color;  // RGB color

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implementing indexing for Vec3 (non-mutable)
impl ops::Index<Vector3> for Vec3 {
    type Output = f64;

    fn index(&self, v: Vector3) -> &Self::Output {
        match v {
            Vector3::X => &self.x,
            Vector3::Y => &self.y,
            Vector3::Z => &self.z,
        }
    }
}

// Implementing indexing for Vec3 (mutable)
impl ops::IndexMut<Vector3> for Vec3 {
    fn index_mut(&mut self, index: Vector3) -> &mut Self::Output {
        match index {
            Vector3::X => &mut self.x,
            Vector3::Y => &mut self.y,
            Vector3::Z => &mut self.z,
        }
    }
}


impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// For commutativity of multiplication, we must have the following implementation as well.
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}


impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    // Dividing a vector by a scalar
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

}



impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Vec3 {
    // Returns a new Vec3 with given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Returns a new Vec3 with all coordinates set to zero
    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    // Helper functions for retrieving x-y-z coordinate values
    pub fn x(&self) -> f64 { return self.x; }
    pub fn y(&self) -> f64 { return self.y; }
    pub fn z(&self) -> f64 { return self.z; }

    // Helper function for calculating distance metric
    pub fn length_squared(&self) -> f64 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }


    // Our metric, for calculating vector length
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    // Calculates the dot product with Vec3 'v'
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        return v / v.length();
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON 
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3 { 
            x: random_double(), 
            y: random_double(), 
            z: random_double(),
        }
    }

    #[inline]
    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::rand_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return Vec3::unit_vector(random_in_unit_sphere());
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere: Vec3 = random_in_unit_sphere();
    if Vec3::dot(&in_unit_sphere, normal) > 0.0 { // In the same hemisphere as the normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0 * (*v).dot(n) * *n;
}