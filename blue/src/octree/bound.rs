use crate::aabb::cube;
use crate::vec3f;
use crate::linked_list;
pub struct obj {
    data: String,
    bound: cube,
}
pub struct node {
    center: vec3f,
    base_length: f32, // Length of this node if it has a looseness of 1.0
    looseness: f32,
    min_size: f32,
    actual_length: f32, // // Actual length of sides, taking the looseness value into account
    bound: cube,
    list_obj: linked_list<obj>,
    children: Option<Box<[node; 8]>>,
    child_bounds: Box<[cube; 8]>,
    cfg_num_objects_allowed: u16,
}
impl node {
    pub fn new(base_length: f32, min_size: f32, looseness: f32, center: vec3f) -> Self {
        let mut new = Self{
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
        new
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
        self.child_bounds[0].center = center.clone() + vec3f::new(-quarter, quarter, -quarter);
        self.child_bounds[1].center = center.clone() + vec3f::new(quarter, quarter, -quarter);
        self.child_bounds[2].center = center.clone() + vec3f::new(-quarter, quarter, quarter);
        self.child_bounds[3].center = center.clone() + vec3f::new(quarter, quarter, quarter);
        self.child_bounds[4].center = center.clone() + vec3f::new(-quarter, -quarter, -quarter);
        self.child_bounds[5].center = center.clone() + vec3f::new(quarter, -quarter, -quarter);
        self.child_bounds[6].center = center.clone() + vec3f::new(-quarter, -quarter, quarter);
        self.child_bounds[7].center = center.clone() + vec3f::new(quarter, -quarter, quarter);
        for i in 0 .. 7 {
            self.child_bounds[i].half_size = child_actual_size;
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
                best_fit_child = self.best_fit_child(&obj.bound.center);
                let child= self.children.as_ref().unwrap().get(best_fit_child as usize).unwrap();
                if node::encapsulates(&child.get_bound(), &obj.bound) {
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
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
            node::new(new_length, self.min_size, self.looseness, vec3f::new_zero()),
        ]));
    }
}