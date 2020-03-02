extern crate raytracer;
use raytracer::Vec3;

pub fn main() {
    let vec = Vec3::zero() + 1.0;
    println!("(0,0,0) + 1 = {}",vec);
}


#[test]
fn  add_test() {
    let vec1 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.5,
    };

    let vec2 = Vec3 {
        x: 0.5,
        y: -1.5,
        z: 0.5,
    };

    let vec3 = vec1 + vec2;

    assert_eq!(
        Vec3 {
            x: 0.5,
            y: -0.5,
            z: 1.0
        },
        vec3
    );
}

#[test]
fn  zero_test() {
    assert_eq!(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        Vec3::zero()
    );
}

#[test]
fn  ortho_test() {
    let vec1 = Vec3 {
        x: 4.0,
        y: 6.0,
        z: 0.0,
    };

    let vec2 = Vec3 {
        x: -3.0,
        y: 2.0,
        z: 0.0,
    };
    assert!(vec1.is_ortho(&vec2));
    assert_eq!(vec1.angle_in_deg(&vec2),90.0);
}