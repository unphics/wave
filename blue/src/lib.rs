mod alloc;
pub use alloc::malloc;
pub use alloc::free;
pub use alloc::deref;
mod list;
pub use list::linked_list;
pub use list::linked_node;

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
