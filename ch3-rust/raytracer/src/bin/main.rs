extern crate raytracer;

use std::fs::File;
use std::io::Write;
use raytracer::vec::Vec3;
use raytracer::ray::Ray;

pub fn color(r : &Ray) -> Vec3 {
    let unit_direction = r.direction().uniform();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * Vec3 { x: 1.0, y: 1.0, z: 1.0,} + 
            t * Vec3 { x: 0.5, y: 0.7, z: 1.0,};
}

pub fn main() -> std::io::Result<()> {
    let mut file = File::create("ch3.ppm")?;
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3 {x: -2.0, y: -1.0, z: -1.0, };
    let horizontal = Vec3 { x: 4.0, y: 0.0, z: 0.0,};
    let vertical = Vec3 { x: 0.0, y: 2.0, z: 0.0,};
    let origin = Vec3::zero();
    let header = format!("P3\n{} {}\n255\n",nx,ny);

    file.write_all(header.as_bytes())?;
    for j in (0..(ny)).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray { a : origin, b : lower_left_corner + u *horizontal + v*vertical };
            let col =color(&r);
            let ir = (255.99*col[0]) as i64;
            let ig = (255.99*col[1]) as i64;
            let ib = (255.99*col[2]) as i64;
            let srgb = format!("{} {} {}\n",ir,ig,ib);
            file.write_all(srgb.as_bytes())?;
        }
    }
    file.sync_all()?;
    Ok(())
}
