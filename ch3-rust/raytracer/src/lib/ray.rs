use std::fmt::{self, Display};
use vec::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    // lerp
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(origin: {}, direction: {})", self.a, self.b,)
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(origin: {}, direction: {})", self.a, self.b,)
    }
}