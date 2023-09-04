pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|e| !Rc::ptr_eq(e, &element));
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    // Return the strong count without subtracting 1.
    Rc::strong_count(ref_list)
}