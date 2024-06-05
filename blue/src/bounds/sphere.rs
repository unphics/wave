use crate::vec3f;
use crate::intersect_data;
pub struct sphere {
    pub center: vec3f,
    pub radius: f32,
}
impl sphere {
    pub fn new(center: vec3f, radius: f32) -> Self {
        Self{center, radius}
    }
    pub fn intersect_sphere(&self, rhs: &sphere) -> intersect_data {
        let radius_dist = self.radius + rhs.radius;
        let center_dist = (rhs.center.clone() - self.center.clone()).len();
        if center_dist < radius_dist {
            return intersect_data::new(true, center_dist - radius_dist);
        } else {
            return intersect_data::new(false, center_dist - radius_dist);
        }
    }
}