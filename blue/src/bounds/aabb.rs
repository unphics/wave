use crate::vec3f;
use crate::intersect_data;
/**
 * @brief 正方体
 */
#[derive(Debug)]
pub struct cube {
    pub center: vec3f,
    pub half_size: f32,
}
impl cube {
    pub fn new(center: vec3f, half_size: f32) -> Self {
        cube {center, half_size}
    }
    pub fn new_zero() -> Self {
        cube::new(vec3f::new_zero(), 0f32)
    }
    pub fn contain(&self, another: &cube) -> bool {
        if self.half_size > another.half_size {
            let disparity_size = (self.half_size - another.half_size);
            let center = another.center.clone();
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
        Self{center: self.center.clone(), half_size: self.half_size}
    }
}
/**
 * @brief 长方体
 */
#[derive(Debug)]
pub struct cuboid {
    pub min_extent: vec3f,
    pub max_extent: vec3f,
}
impl cuboid {
    pub fn new(min_extent: vec3f, max_extent: vec3f) -> Self {
        cuboid{min_extent, max_extent}
    }
    pub fn interesct_cuboid(&self, rhs: &Self) -> intersect_data {
        // todo 没懂
        let dist_1 = rhs.min_extent.clone() - self.max_extent.clone();
        let dist_2 = self.min_extent.clone() - rhs.max_extent.clone();
        let dist = dist_1.max(&dist_2);
        let max_dist = dist.max_value();
        return intersect_data::new(max_dist < 0f32, max_dist);
    }
}
impl Clone for cuboid {
    fn clone(&self) -> Self {
        Self{min_extent: self.min_extent.clone(), max_extent: self.max_extent.clone()}
    }
}