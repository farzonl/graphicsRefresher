extern crate raytracer;
use raytracer::vec::Vec3;
use raytracer::ray::Ray;

pub fn main() {
    let vec = Vec3::zero() + 1.0;
    println!("(0,0,0) + 1 = {}",vec);
    let ray = Ray {
        origin: Vec3::zero(),
        direction: vec
    };
    let pointing_at = ray.point_at_parameter(2.0);
    println!("{}",pointing_at);
}