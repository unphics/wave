use crate::math::vec::vec3f;
use super::{intersect_data::intersect_data, sphere::sphere};
pub struct plane {
    normal: vec3f,
    distance: f32,
}
impl plane {
    pub fn new(normal: vec3f, distance: f32) -> Self {
        return plane {
            normal: normal,
            distance: distance,
        };
    }
    pub fn normal(&self) -> vec3f {
        return self.normal.clone();
    }
    pub fn distance(&self) -> f32 {
        return self.distance;
    }
    pub fn normalized(&self) -> plane {
        let magnitude = self.normal().len(); // 幅度
        return plane::new(self.normal() / magnitude, self.distance() / magnitude);
    }
    // pub fn intersect_sphere(&self, sphere: &sphere) -> intersect_data {
    //     let dist_from_sphere_center = (self.normal().dot(&sphere.center()) + self.distance()).abs();
        
    // }
}