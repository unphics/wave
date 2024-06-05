mod alloc;
pub use alloc::malloc;
pub use alloc::free;
pub use alloc::deref;
mod list;
pub use list::linked_list;
pub use list::linked_node;
mod math;
pub use math::vec3f::vec3f;
mod bounds;
pub use bounds::intersect_data::intersect_data;
pub use bounds::aabb;
pub use bounds::sphere::sphere;
mod octree;
pub use octree::bound as octree_bound;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
