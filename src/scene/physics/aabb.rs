use crate::math::vec::vec3f;
use super::intersect_data::intersect_data;
pub struct aabb {
    min_extent: vec3f,
    max_extent: vec3f,
}
impl aabb {
    pub fn new(min_extent: vec3f, max_extent: vec3f) -> Self {
        return aabb{min_extent: min_extent, max_extent: max_extent};
    }
    pub fn min_extent(&self) -> vec3f {
        return self.min_extent.clone();
    }
    pub fn max_extent(&self) -> vec3f {
        return self.max_extent.clone();
    }
    pub fn interesct_aabb(&self, other: aabb) -> intersect_data {
        // todo 没懂
        let dist_1 = other.min_extent() - self.max_extent();
        let dist_2 = self.min_extent() - other.max_extent();
        let dist = dist_1.max(&dist_2);
        let max_dist = dist.max_value();
        return intersect_data::new(max_dist < 0f32, max_dist);
    }
}
impl Clone for aabb {
    fn clone(&self) -> Self {
        aabb{min_extent: self.min_extent.clone(), max_extent: self.max_extent.clone()}
    }
}
#[derive(Debug)]
pub struct cube {
    center: vec3f,
    half_size: f32,
}
impl cube {
    pub fn new(center: vec3f, half_size: f32) -> Self {
        cube {center, half_size}
    }
    pub fn new_zero() -> Self {
        cube::new(vec3f::new_zero(), 0f32)
    }
    pub fn set_center(&mut self, center: vec3f) {
        self.center = center;
    }
    pub fn set_half_size(&mut self, half_size: f32) {
        self.half_size = half_size;
    }
    pub fn half_size(&self) -> f32 {
        self.half_size
    }
    pub fn center(&self) -> vec3f {
        self.center.clone()
    }
    pub fn contain(&self, another: &cube) -> bool {
        if self.half_size > another.half_size() {
            let disparity_size = (self.half_size - another.half_size());
            let center = another.center();
            return self.center.x + disparity_size > center.x && self.center.x - disparity_size < center.x &&
                self.center.y + disparity_size > center.y &&  self.center.y - disparity_size < center.y &&
                self.center.z + disparity_size > center.z &&  self.center.z - disparity_size < center.z;
        } else {
            return false;
        }
    }
}
impl Clone for cube {
    fn clone(&self) -> Self {
        cube{center: self.center.clone(), half_size: self.half_size}
    }
}