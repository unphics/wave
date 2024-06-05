use std::ops::Sub;
use std::ops::Div;
use std::ops::Add;
#[derive(Debug)]
#[repr(C)]
pub struct vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }
    pub fn new_zero() -> Self {
        vec3f::new(0f32, 0f32, 0f32)
    }
    pub fn len(&self) -> f32 {
        self.dot(&self).sqrt()
    }
    pub fn max(&self, other: &Self)  -> Self {
        let mut result = Self::new_zero();
        result.x = if self.x > other.x {self.x} else {other.x};
        result.y = if self.y > other.y {self.y} else {other.y};
        result.z = if self.z > other.z {self.z} else {other.z};
        result
    }
    pub fn max_value(&self) -> f32 {
        if self.x > self.y {if self.x > self.z {self.x} else {self.z}} else {if self.y > self.z {self.y} else {self.z}}
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl Clone for vec3f {
    fn clone(&self) -> Self {
        Self {x: self.x, y: self.y, z: self.z}
    }
}
impl Add<vec3f> for vec3f {
    type Output = vec3f;
    fn add(self, rhs: vec3f) -> vec3f {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
impl Sub<vec3f> for vec3f {
    type Output = vec3f;
    fn sub(self, rhs: vec3f) -> vec3f {
        Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}
impl Div<f32> for vec3f {
    type Output = vec3f;
    fn div(self, rhs: f32) -> vec3f {
        return vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}