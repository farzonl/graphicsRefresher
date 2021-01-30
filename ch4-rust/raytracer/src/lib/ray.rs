use std::fmt::{self, Display};
use Vec3;

#[derive(Clone, Copy, Default)] //to solve value used here after move error
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn point_at_parameter(self, t: f64) -> Vec3  {
        self.origin + t * self.direction
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(origin: {}, direction: {})", self.origin, self.direction,)
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(origin: {}, direction: {})", self.origin, self.direction,)
    }
}