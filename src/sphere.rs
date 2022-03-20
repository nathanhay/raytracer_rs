use crate::hittable::*;
use crate::vector3::vec3::*;
use crate::ray::Ray;
use crate::material::Scatterable;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Scatterable>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Scatterable>) -> Sphere {
        Sphere { center: cen, radius: r, mat_ptr: m }
    } 
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { 
            return None 
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { 
                return None 
            }
        }

        let mut rec: Option<HitRecord> = Some(HitRecord{
            p: r.at(root),
            normal: r.at(root),
            mat_ptr: Arc::clone(&self.mat_ptr),
            t: root,
            front_face: false
        });

        let outward_normal: Vec3 = (rec.as_ref().unwrap().p - self.center) / self.radius;
        rec.as_mut().unwrap().set_face_normal(r, outward_normal);

        return rec;
    }
}