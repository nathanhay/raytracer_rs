use rand::prelude::*;

#[inline]
pub fn random_double() -> f64 {
    // Returns a random real in [0, 1)
    rand::thread_rng().gen_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min, max)
    rand::thread_rng().gen_range(min..max)
}

#[inline]
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}