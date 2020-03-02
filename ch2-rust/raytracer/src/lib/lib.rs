
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt::{self, Display};
use std::cmp;



#[derive(Clone, Copy, Default)] //to solve value used here after move error
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn r(&self) -> &f64 {
        &self.x
    }

    pub fn g(&self) -> &f64 {
        &self.y
    }
    pub fn b(&self) -> &f64 {
        &self.z
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn dot(&self, r: &Vec3) -> f64 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }

    pub fn cross(&self, r: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * r.z - self.z * r.y,
            y: self.z * r.x - self.x * r.z,
            z: self.x * r.y - self.y * r.x,
        }
    }

    pub fn is_ortho(&self,r : &Vec3) -> bool {
        self.dot(r) == 0.0
    }

    pub fn angle(self, r: &Vec3) -> f64 {
        let numerator = self.dot(r);
        let denominator = self.mag() * r.mag();
        return (numerator / denominator).acos();
    }

    pub fn angle_in_deg(self, r: &Vec3) -> f64 {
        self.angle(r).to_degrees()
    }

    pub fn uniform(&self) -> Vec3 {
        let mag = self.mag();

        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, r: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, r: f64) -> Vec3 {
        Vec3 {
            x: self.x + r,
            y: self.y + r,
            z: self.z + r,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, r: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, r: f64) -> Vec3 {
        Vec3 {
            x: self.x - r,
            y: self.y - r,
            z: self.z - r,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, r: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * r.x,
            y: self.y * r.y,
            z: self.z * r.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, r: f64) -> Vec3 {
        Vec3 {
            x: self.x * r,
            y: self.y * r,
            z: self.z * r,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, r: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / r.x,
            y: self.y / r.y,
            z: self.z / r.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, r: f64) -> Vec3 {
        Vec3 {
            x: self.x / r,
            y: self.y / r,
            z: self.z / r,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, r: &Vec3) -> bool {
        self.x == r.x && self.y == r.y && r.z == r.z
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}