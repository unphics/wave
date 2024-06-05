use crate::math::vec::vec3f;
use super::aabb::cube;
use blue::{self, linked_list};

struct obj {
    data: String,
    bound: cube,
}

pub struct bound_node {
    center: vec3f,
    base_length: f32, // Length of this node if it has a looseness of 1.0
    looseness: f32,
    min_size: f32,
    actual_length: f32, // // Actual length of sides, taking the looseness value into account
    bound: cube,
    list_obj: blue::linked_list<obj>,
    children: Option<Box<[bound_node; 8]>>,
    child_bounds: Box<[cube; 8]>,
    cfg_num_objects_allowed: u16,
}
impl bound_node {
    pub fn new(base_length: f32, min_size: f32, looseness: f32, center: vec3f) -> Self {
        let mut new = bound_node{
            center: vec3f::new_zero(),
            base_length: 0f32,
            min_size: 0f32,
            looseness: 0f32,
            actual_length: 0f32,
            bound: cube::new_zero(),
            list_obj: linked_list::new(),
            children: None,
            child_bounds: Box::new(std::array::from_fn(|_| cube::new_zero())),
            cfg_num_objects_allowed: 8,
        };
        new.set_value(base_length, min_size, looseness, center);
        return new;
    }
    pub fn get_center(&self) -> vec3f {
        self.center.clone()
    }
    pub fn get_bound(&self) -> cube {
        self.bound.clone()
    }
    pub fn has_children(&self) -> bool {
        self.children.is_none()
    }
    /**
     * @brief set values for this node
     * @param base_length: Length of this node, not taking looseness into account
     * @param min_size: Minimum size of nodes in this octree
     * @param losseness: Multiplier for baseLengthVal to get the actual size
     * @param center: Centre position of this node
     */
    pub fn set_value(&mut self, base_length: f32, min_size: f32, looseness: f32, center: vec3f) {
        self.base_length = base_length;
        self.min_size = min_size;
        self.looseness = looseness;
        self.center = center.clone();
        self.actual_length = looseness * base_length;
        self.bound = cube::new(center.clone(), self.actual_length);
        let quarter = base_length / 4f32; // quarter: 四分之一
        let child_actual_size = base_length / 2f32 * looseness;
        self.child_bounds[0].set_center(center.clone() + vec3f::new(-quarter, quarter, -quarter));
        self.child_bounds[1].set_center(center.clone() + vec3f::new(quarter, quarter, -quarter));
        self.child_bounds[2].set_center(center.clone() + vec3f::new(-quarter, quarter, quarter));
        self.child_bounds[3].set_center(center.clone() + vec3f::new(quarter, quarter, quarter));
        self.child_bounds[4].set_center(center.clone() + vec3f::new(-quarter, -quarter, -quarter));
        self.child_bounds[5].set_center(center.clone() + vec3f::new(quarter, -quarter, -quarter));
        self.child_bounds[6].set_center(center.clone() + vec3f::new(-quarter, -quarter, quarter));
        self.child_bounds[7].set_center(center.clone() + vec3f::new(quarter, -quarter, quarter));
        for i in 0 .. 7 {
            self.child_bounds[i].set_half_size(child_actual_size);
        }
    }
    pub fn add(&mut self, obj: obj, bound: cube) -> bool {
        if !self.bound.contain(&bound) {
            return false;
        }
        self.sub_add(obj, bound);
        true
    }
    pub fn sub_add(&mut self, obj: obj, bound: cube) { // todo 这俩参数其实是一个参数
        if !self.has_children() {
            if self.list_obj.len() < self.cfg_num_objects_allowed as u32 || (self.base_length / 2 as f32) < self.min_size { // todo 
                self.list_obj.insert_back(obj);
                return;
            }
            let mut best_fit_child = 0;
            if self.children.is_none() {
                self.split();
                if self.children.is_none() {
                    eprintln!("failed to create children");
                    return;
                }
            }
            self.list_obj.foreach(|node| {
                let obj = node.data.as_ref().unwrap();
                best_fit_child = self.best_fit_child(&obj.bound.center());
                let child= self.children.as_ref().unwrap().get(best_fit_child as usize).unwrap();
                if bound_node::encapsulates(&child.get_bound(), &obj.bound) {
                    // child.add(obj., bound)
                    // todo last 不行喵的不能盲目的抄, 明天得好好看看这个数据结构
                }
            });
        }
    }
    pub fn encapsulates(outer: &cube, inner: &cube) -> bool {
        return outer.contain(inner);
    }
    pub fn best_fit_child(&self, center: &vec3f) -> i32 {
        return (if center.x <= self.center.x {0} else {1}) + (if center.y >= self.center.y {0} else {4}) + (if center.z <= self.center.z {0} else {2});
    }
    fn split(&mut self) {
        let quarter = self.base_length / 4f32;
        let new_length = self.base_length / 2f32;
        self.children = Some(Box::new([
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            bound_node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
        ]));
    }
}


/*
#[derive(Debug)]
struct aabb {
    center: vec3f,
    half_size: f32,
}
impl aabb {
    fn new(center: vec3f, half_size: f32) -> Self {
        aabb{center, half_size}
    }
    fn is_contains_point(&self, point: &vec3f) -> bool {
        (point.x >= self.center.x - self.half_size) && (point.x <= self.center.x + self.half_size) &&
        (point.y >= self.center.y - self.half_size) && (point.y <= self.center.y + self.half_size) &&
        (point.z >= self.center.z - self.half_size) && (point.z <= self.center.z + self.half_size)
    }
    fn is_intersects_aabb(&self, other: &aabb) -> bool {
        (self.center.x - self.half_size <= other.center.x + other.half_size) &&
        (self.center.x + self.half_size >= other.center.x - other.half_size) &&
        (self.center.y - self.half_size <= other.center.y + other.half_size) &&
        (self.center.y + self.half_size >= other.center.y - other.half_size) &&
        (self.center.z - self.half_size <= other.center.z + other.half_size) &&
        (self.center.z + self.half_size >= other.center.z - other.half_size)
    }
}
#[derive(Debug)]
struct node {
    bound: aabb,
    points: Vec<vec3f>,
    children: [*mut node; 8],
    capacity: usize,
}
impl node {
    fn new(bound: aabb, capacity: usize) -> Self {
        node {bound, points: Vec::new(), children: [std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut(),std::ptr::null_mut()], capacity}
    }
    fn subdivide(&mut self) {
        let new_half_size = self.bound.half_size / 2.0;
        let centers = [
            vec3f { x: self.bound.center.x - new_half_size, y: self.bound.center.y - new_half_size, z: self.bound.center.z - new_half_size },
            vec3f { x: self.bound.center.x + new_half_size, y: self.bound.center.y - new_half_size, z: self.bound.center.z - new_half_size },
            vec3f { x: self.bound.center.x - new_half_size, y: self.bound.center.y + new_half_size, z: self.bound.center.z - new_half_size },
            vec3f { x: self.bound.center.x + new_half_size, y: self.bound.center.y + new_half_size, z: self.bound.center.z - new_half_size },
            vec3f { x: self.bound.center.x - new_half_size, y: self.bound.center.y - new_half_size, z: self.bound.center.z + new_half_size },
            vec3f { x: self.bound.center.x + new_half_size, y: self.bound.center.y - new_half_size, z: self.bound.center.z + new_half_size },
            vec3f { x: self.bound.center.x - new_half_size, y: self.bound.center.y + new_half_size, z: self.bound.center.z + new_half_size },
            vec3f { x: self.bound.center.x + new_half_size, y: self.bound.center.y + new_half_size, z: self.bound.center.z + new_half_size },
        ];

    }
} */