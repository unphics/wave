use crate::deref;
use crate::malloc;
use crate::octree_bound_obj as obj;
use crate::octree_bound_node as node;
use crate::aabb::cube;
use crate::vec3f;
use crate::linked_list;

use super::node::octree_bound_node;

pub struct container {
    count: u64,
    root_node: *mut node,
    looseness: f32,
    init_size: f32,
    min_size: f32,
}
pub trait octree_bound_container {
    fn new(init_size: f32, init_world_pos: vec3f, min_node_size: f32, looseness: f32) -> Self;
    fn add(&mut self, obj: *mut obj);
    fn remove_obj(&mut self, obj: *mut obj) -> bool;
    // fn remove_bound(&mut self, obj: *mut obj, bound: & cube) -> bool;
    fn is_colliding_bound(&self, bound: &cube) -> bool;
    // fn is_colliding_ray(&self) -> bool;
    fn get_colliding_bound(&self, bound: &cube, out_objs: &mut linked_list<*mut obj>);
    // fn get_colliding_ray(&self);
    fn get_max_bound(&self) -> cube;
    fn grow(&mut self, dir: &vec3f);
    fn shrink(&mut self);
}
impl octree_bound_container for container {
    fn new(init_size: f32, init_world_pos: vec3f, min_node_size: f32, looseness: f32) -> Self {
        let mut actual_min_node_size = min_node_size;
        if min_node_size > init_size {
            eprintln!("Minimum node size must be at least as big as the initial world size");
            actual_min_node_size = init_size;
        }
        Self {
            count: 0,
            init_size,
            min_size: actual_min_node_size,
            looseness, // 1 <= losseness <= 2
            root_node: malloc(node::new(init_size, min_node_size, looseness, init_world_pos)),
        }
    }
    fn add(&mut self, obj: *mut obj) {
        deref(self.root_node).add(obj, deref(obj).bound.clone());
        self.count += 1;
    }
    fn remove_obj(&mut self, obj: *mut obj) -> bool {
        let removed = deref(self.root_node).remove_obj(obj);
        if removed {
            self.shrink();
        }
        removed
    }
    fn is_colliding_bound(&self, bound: &cube) -> bool {
        return deref(self.root_node).is_colliding_bound(bound);
    }
    fn get_colliding_bound(&self, bound: &cube, out_objs: &mut linked_list<*mut obj>) {
        deref(self.root_node).get_colling_bound(bound,  out_objs);
    }
    fn get_max_bound(&self) -> cube {
        deref(self.root_node).get_bound()
    }
    fn grow(&mut self, dir: &vec3f) {
        let x_dir = if dir.x >= 0f32 {1} else {-1};
        let y_dir = if dir.y >= 0f32 {1} else {-1};
        let z_dir = if dir.z >= 0f32 {1} else {-1};
        todo!();
    }
    fn shrink(&mut self) {
        todo!();
    }
}