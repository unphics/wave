use crate::math::vec::vec3f;

use super::intersect_data::intersect_data;

pub struct sphere {
    center: vec3f,
    radius: f32,
}
impl sphere {
    pub fn new(center: vec3f, raduis: f32) -> Self {
        return sphere {
            center: center,
            radius: raduis,
        };
    }
    pub fn center(&self) -> vec3f {
        return self.center.clone();
    }
    pub fn radius(&self) -> f32 {
        return self.radius;
    }
    pub fn intersect_sphere(&mut self, other: &sphere) -> intersect_data {
        let radius_dist = self.radius + other.radius();
        let center_dist = (other.center() - self.center()).len();
        if center_dist < radius_dist {
            return intersect_data::new(true, center_dist - radius_dist);
        } else {
            return intersect_data::new(false, center_dist - radius_dist);
        }
    }
}