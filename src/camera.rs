use crate::vector3::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let orig = Point3 {x: 0.0, y: 0.0, z: 0.0};
        let vert = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
        let horiz = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
        let llc = orig - horiz/2.0 - vert/2.0 - Vec3 {x: 0.0, y: 0.0, z: focal_length};

        Camera {
            origin: orig,
            lower_left_corner: llc,
            horizontal: horiz,
            vertical: vert,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}
