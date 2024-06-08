use std::char::ToLowercase;
use std::iter::Inspect;
use std::os::unix::raw::ino_t;
use crate::deref;
use crate::free;
use crate::malloc;
use crate::aabb::cube;
use crate::vec3f;
use crate::linked_list;
pub struct obj {
    data: String,
    pub bound: cube,
}
pub struct node {
    center: vec3f,
    base_length: f32, // Length of this node if it has a looseness of 1.0
    looseness: f32,
    min_size: f32,
    actual_length: f32, // // Actual length of sides, taking the looseness value into account
    bound: cube,
    list_obj: linked_list<*mut obj>,
    children: [*mut node; 8],
    child_bounds: Box<[cube; 8]>,
    cfg_num_objects_allowed: u16,
}
pub trait octree_node_bound {
    fn new(base_length: f32, min_size: f32, looseness: f32, center: vec3f) -> Self;
    /**
     * @brief: add an object
     */
    fn add(&mut self, obj: *mut obj, bound: cube) -> bool;
    /**
     * @brief: remove an object
     * @notice: makes the assumption that the obj only exist once in the tree
     */
    fn remove_obj(&mut self, obj: *mut obj) -> bool;
    fn has_children(&self) -> bool;
    /**
     * @brief: check if the bound intersect with anything in the tree
     */
    fn is_colliding_bound(&self, rhs: &cube) -> bool;
    // fn is_colliding_ray(&self, rhs: &cube) -> bool;
    /**
     * @brief: return an array of objects that intersect with the bound
     */
    fn get_colling_bound(&self, bound: &cube, out_list: &mut linked_list<*mut obj>);
    // fn get_colling_ray(&self, bound: &cube, out_list: linked_list<*mut obj>);
    fn get_bound(&self) -> cube;
    /**
     * @biref: shrink the octree if: (收缩八叉树)
     *      1. this node is >= double min_length in length
     *      2. all objects in the root node
     *      3. this node has no children, or most child is empty
     */
    fn shrink_if_possibe(&mut self, min_length: f32);
    /**
     * @brief: find which child node this object would be most likely to fit in
     */
    fn best_fit_child(&self, bound_center: &vec3f) -> usize;
    fn has_any_obj(&self) -> bool;
    fn _set_value(&mut self, base_length: f32, min_size: f32, looseness: f32, center: vec3f);
    fn _sub_add(&mut self, obj: *mut obj, bound: &cube);
    /**
     * @brief: splits the octree into eight children
     */
    fn _split(&mut self);
    /**
     * @brief: merge all children into this node
     */
    fn _merge(&mut self);
    /**
     * @brief check if outer_bound encapsulates(包含) inner_bound
     */
    fn _encapsulates(outer: &cube, inner: &cube) -> bool;
    fn _should_merge(&self) -> bool;
}
impl octree_node_bound for node {
    fn new(base_length: f32, min_size: f32, looseness: f32, center: vec3f) -> Self {
        let mut new = Self{
            center: vec3f::new_zero(),
            base_length: 0f32,
            min_size: 0f32,
            looseness: 0f32,
            actual_length: 0f32,
            bound: cube::new_zero(),
            list_obj: linked_list::new(),
            children: [std::ptr::null_mut(); 8],
            child_bounds: Box::new(std::array::from_fn(|_| cube::new_zero())),
            cfg_num_objects_allowed: 8,
        };
        new._set_value(base_length, min_size, looseness, center);
        new
    }
    fn get_bound(&self) -> cube {
        self.bound.clone()
    }
    fn has_children(&self) -> bool {
        self.children[0] == std::ptr::null_mut()
    }
    fn get_colling_bound(&self, bound: &cube, out_list: &mut linked_list<*mut obj>) {
        if !self.bound.intersect_cube(bound) {
            return;
        }
        self.list_obj.foreach(|node| {
            if deref(node.data.unwrap()).bound.intersect_cube(bound) {
                out_list.insert_back_node(node);
            }
        });
        if self.has_children() {
            for child in self.children {
                deref(child).get_colling_bound(bound, out_list);
            }
        }
    }
    fn has_any_obj(&self) -> bool {
        if self.list_obj.len() > 0 {return true}
        if self.has_children() {
            for child in self.children {
                if deref(child).has_children() {
                    return true
                }
            }
        }
        return false;
    }
    fn shrink_if_possibe(&mut self, min_length: f32) {
        if self.base_length < 2f32 * min_length {
            return;
        }
        if self.list_obj.len() == 0 && !self.has_children() {
            return;
        }
        // check objects in root
        let best_fit = -1;
        todo!();
    }
    /**
     * @brief set values for this node
     * @param base_length: Length of this node, not taking looseness into account
     * @param min_size: Minimum size of nodes in this octree
     * @param losseness: Multiplier for baseLengthVal to get the actual size
     * @param center: Centre position of this node
     */
    fn _set_value(&mut self, base_length: f32, min_size: f32, looseness: f32, center: vec3f) {
        self.base_length = base_length;
        self.min_size = min_size;
        self.looseness = looseness;
        self.center = center.clone();
        self.actual_length = looseness * base_length;
        self.bound = cube::new(center.clone(), self.actual_length);
        let quarter = base_length / 4f32; // quarter: 1/4
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
    fn add(&mut self, obj: *mut obj, bound: cube) -> bool {
        if !self.bound.contain(&bound) {
            return false;
        }
        self._sub_add(obj, &bound);
        true
    }
    fn remove_obj(&mut self, obj: *mut obj) -> bool {
        let mut removed = false;

        // first, check this node self
        self.list_obj.foreach(|node| {
            if node.data.unwrap() == obj {
                node.cut_self();
                free(node);
                removed = true;
            }
        });

        // second, check children of this node
        if !removed && self.has_children() {
            for child in self.children {
                removed = deref(child).remove_obj(obj);
                if removed {
                    break;
                }
            }
        }
        if removed && self.has_children() {
            if self._should_merge() {
                self._merge();
            }
        }
        removed
    }
    fn is_colliding_bound(&self, rhs: &cube) -> bool {
        if self.bound.intersect_cube(rhs) { return false; }
        let mut ret = false;
        self.list_obj.foreach(|node| {
            let obj = deref(node.data.unwrap());
            if obj.bound.intersect_cube(rhs) {
                ret = true;
            }
        });
        if ret {return true;}
        if self.has_children() {
            for child in self.children {
                if deref(child).is_colliding_bound(rhs) {
                    return true;
                }
            }
        }
        false
    }
    fn _should_merge(&self) -> bool {
        let mut total_objs = self.list_obj.len();
        if self.has_children() {
            for child in self.children {
                if deref(child).has_children() {
                    return false;
                }
                total_objs += deref(child).list_obj.len();
            }
        }
        return total_objs <= self.cfg_num_objects_allowed as u32;
    }
    fn _merge(&mut self) {
        for child in self.children {
            let num_objs = deref(child).list_obj.len();
            deref(child).list_obj.foreach(|node| {
                node.cut_self();
                self.list_obj.insert_back_node(node);
                free(node);
            });
            free(child);
        }
        self.children = [std::ptr::null_mut(); 8];
    }
    fn _sub_add(&mut self, in_obj: *mut obj, in_obj_bound: &cube) { // todo 这俩参数其实是一个参数
        if !self.has_children() {
            if self.list_obj.len() < self.cfg_num_objects_allowed as u32 || (self.base_length / 2 as f32) < self.min_size { // todo 
                self.list_obj.insert_back(in_obj);
                return;
            }
            let mut best_fit_child = 0;
            if !self.has_children() {
                self._split();
                if !self.has_children() {
                    eprintln!("failed to create children");
                    return;
                }
            }
            self.list_obj.foreach(|node| {
                let exist_obj = node.data.as_ref().unwrap();
                best_fit_child = self.best_fit_child(&deref(*exist_obj).bound.center);
                let child = self.children[best_fit_child];
                let child = deref(child);
                if node::_encapsulates(&child.get_bound(), &deref(*exist_obj).bound) {
                    // todo last 不行喵的不能盲目的抄, 明天得好好看看这个数据结构
                    child.add(*exist_obj, deref(*exist_obj).bound.clone());
                    node.cut_self(); // 从self.list_obj中移除
                }
            });
        }
        let best_fit = self.best_fit_child(&in_obj_bound.center);
        let best_fit_child = deref(self.children[best_fit]);
        if node::_encapsulates(&best_fit_child.bound, &in_obj_bound) {
            best_fit_child._sub_add(in_obj, &in_obj_bound);
        } else {
            // did not fit in a child, so we will insert it to this node
            // let new_obj = malloc(in_obj);
            self.list_obj.insert_back(in_obj);
        }
    }
    fn _encapsulates(outer: &cube, inner: &cube) -> bool {
        return outer.contain(inner);
    }
    fn best_fit_child(&self, center: &vec3f) -> usize {
        return (if center.x <= self.center.x {0} else {1}) + (if center.y >= self.center.y {0} else {4}) + (if center.z <= self.center.z {0} else {2});
    }
    fn _split(&mut self) {
        let quarter = self.base_length / 4f32;
        let new_length = self.base_length / 2f32;
        self.children = [malloc(node::new(new_length, self.min_size, self.looseness, vec3f::new_zero())); 8];
    }
}