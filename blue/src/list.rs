use super::malloc;
use super::free;
use super::deref;
pub struct linked_node<T> {
    pub data: Option<T>,
    pub prev: *mut linked_node<T>,
    pub next: *mut linked_node<T>,
}
impl<T> linked_node<T> {
    pub fn new(data: T) -> *mut Self {
        malloc(linked_node {data: Some(data), prev: std::ptr::null_mut(), next: std::ptr::null_mut()})
    }
    pub fn new_null() -> *mut Self {
        malloc(linked_node {data: None, prev: std::ptr::null_mut(), next: std::ptr::null_mut()})
    }
    pub fn remove_self(&mut self) {
        deref(self).cut_self();
        free(self);
    }
    pub fn cut_self(&self) {
        if self.prev != std::ptr::null_mut() {deref(self.prev).next = self.next}
        if self.next != std::ptr::null_mut() {deref(self.next).prev = self.prev}
    }
    pub fn insert(&mut self, node: *mut Self) {
        deref(node).next = self.next;
        if self.next != std::ptr::null_mut() {deref(self.next).prev = node}
        deref(node).prev = self;
        self.next = node;
    }
}
pub struct linked_list<T> {
    sentinel: *mut linked_node<T>,
}
impl<T> linked_list<T> {
    pub fn new() -> Self {
        let list = linked_list{
            sentinel: linked_node::new_null(),
        };
        deref(list.sentinel).next = list.sentinel;
        deref(list.sentinel).prev = list.sentinel;
        return list;
    }
    pub fn sentinel(&self) -> *mut linked_node<T> {
        self.sentinel
    }
    pub fn insert_back(&self, data: T) {
        self.insert_back_node(linked_node::new(data));
    }
    pub fn insert_back_node(&self, node: *mut linked_node<T>) {
        deref(deref(self.sentinel).prev).insert(node);
    }
    pub fn foreach<F: FnMut(&mut linked_node<T>)>(&self, mut f: F) {
        let mut cur: *mut linked_node<T> = deref(self.sentinel).next;
        while cur != self.sentinel {
            let next: *mut linked_node<T> = deref(cur).next;
            f(deref(cur));
            cur = next;
        }
    }
    pub fn len(&self) -> u32 {
        let mut cur = deref(self.sentinel).next;
        let mut len = 0;
        while cur != self.sentinel {
            len = len + 1;
            cur = deref(cur).next;
        }
        return len;
    }
}
impl<T> Drop for linked_list<T> {
    fn drop(&mut self) {
        let mut cur = deref(self.sentinel).next;
        while cur != self.sentinel {
            let next = deref(cur).next;
            deref(cur).remove_self();
            cur = next;
        }
        free(self.sentinel);
    }
}

#[cfg(test)]
mod tests {
    use crate::malloc;
    use super::linked_node;
    use super::linked_list;
    use crate::deref;

    #[test]
    fn list_work_test() {
        let list = linked_list::new();
        list.insert_back(15);
        list.insert_back(17);
        list.insert_back(88);
        assert_eq!(list.len(), 3);
        let mut count = 2;
        list.foreach(move |node| {
            if count != 0 {
                node.remove_self();
                count = count - 1;
            } else {
                assert_eq!(node.data.unwrap(), 88);
            }
        });
        assert_eq!(list.len(), 1);
        let node = deref(list.sentinel()).next;
        deref(node).cut_self();
        let list2 = linked_list::new();
        list2.insert_back_node(node);
        assert_eq!(list.len(), 0);
        assert_eq!(list2.len(), 1);
        assert_eq!(deref(deref(list2.sentinel()).next).data.unwrap(), 88);
    }
}