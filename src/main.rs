mod vector3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod utils;
mod camera;
mod material;
use crate::camera::*;
use crate::ray::Ray;
use crate::vector3::vec3::*;
use crate::vector3::color::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::sphere::*;
use crate::utils::*;
use crate::material::*;
use std::sync::Arc;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Point3 = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(&oc, &r.direction());
    // let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = oc.length_squared()- radius*radius;
    let discriminant = half_b*half_b - a*c;
    
    // computing and visualizing the surface normal for shading
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (2.0*a);
    }
}


fn ray_color(r: &Ray, world: &dyn Hittable, depth: i64) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered/
    if depth <= 0 {
        return Color {x: 0.0, y: 0.0, z: 0.0};
    }


    // using t = 0.001 to ignore hits very near zero and avoid shadow acne
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let scattered = rec.mat_ptr.scatter(&r, &rec);
        match scattered {
            Some((scattered_ray, attenuation)) => {
                return attenuation * ray_color(&scattered_ray, world, depth-1);
            }
            None => {
                return Color::new(0.0,0.0,0.0);
            }
        }
       
        // let target: Point3 = rec.p + random_in_hemisphere(&rec.normal);
        // return 0.5 * ray_color(&Ray { orig: rec.p, dir: target - rec.p }, world, depth-1);
        // return 0.5 * (rec.normal + Color {x: 1.0, y: 1.0, z: 1.0});
    }
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0)*(1.0 - t) + Color::new(0.5, 0.7, 1.0)*t; 
}


fn main() {
    // Image
    let image_width = 400;
    let image_height = 225; // image_width / aspect_ratio as i64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground: Arc<Lambertian> = Arc::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });
    let material_center: Arc<Lambertian> = Arc::new(Lambertian { albedo: Color::new(0.7, 0.3, 0.3) });
    let material_left: Arc<Metal> = Arc::new(Metal { albedo: Color::new(0.8, 0.8, 0.8) });
    let material_right: Arc<Metal> = Arc::new(Metal { albedo: Color::new(0.8, 0.6, 0.2) });


    world.add(Arc::new(Sphere {center: Point3 { x: 0.0, y: 0.0, z: -1.0}, radius: 0.5, mat_ptr: material_center}));
    world.add(Arc::new(Sphere {center: Point3 { x: 0.0, y: -100.5, z: -1.0}, radius: 100.0, mat_ptr: material_ground}));
    world.add(Arc::new(Sphere {center: Point3 { x: -1.0, y: 0.0, z: -1.0}, radius: 0.5, mat_ptr: material_left}));
    world.add(Arc::new(Sphere {center: Point3 { x: 1.0, y: 0.0, z: 1.0}, radius: 0.5, mat_ptr: material_right}));




    // Camera
    let cam: Camera = Camera::new();

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut j = image_height - 1;
    let denom_i = image_width - 1;
    let denom_j = image_height - 1;


    while j >= 0 {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color {x: 0.0, y: 0.0, z: 0.0};
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double())/denom_i as f64;
                let v = (j as f64 + random_double())/denom_j as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
        j -= 1;
    }

    eprintln!("Done.");
}