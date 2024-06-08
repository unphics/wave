use crate::octree_bound_obj as obj;
use crate::octree_bound_node as node;
use crate::aabb::cube;
use crate::vec3f;
use crate::linked_list;

pub struct container {
    count: u64,
    root_node: node,
    looseness: f32,
    init_size: f32,
    min_size: f32,
}
pub trait octree_bound_container {
    fn new(init_size: f32, init_world_pos: vec3f, min_noed_size: f32, looseness: f32) -> Self;
    fn add(&mut self, obj: *mut obj);
    fn remove_obj(&mut self, obj: *mut obj) -> bool;
    // fn remove_bound(&mut self, obj: *mut obj, bound: & cube) -> bool;
    fn is_colliding_bound(&self, bound: &cube) -> bool;
    // fn is_colliding_ray(&self) -> bool;
    fn get_colliding_bound(&self, bound: &cube, out_objs: linked_list<*mut obj>);
    // fn get_colliding_ray(&self);
    fn get_max_bound(&self) -> cube;
    fn grow(&mut self, dir: &vec3f);
    fn shrink(&mut self);
}