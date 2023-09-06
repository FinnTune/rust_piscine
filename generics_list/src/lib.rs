#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
       let new_node = Box::new(Node {
           value,
           next: self.head.take(),
       });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        self.head = self.head.take().and_then(|node| node.next);
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            count += 1;
            current = &node.next;
        }
        count
    }
}