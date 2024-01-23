struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct MyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> MyLinkedList<T> {
    pub fn new() -> Self {
        MyLinkedList { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn peek_at(&self, index: usize) -> Option<&T> {
        let mut current_node = &self.head;
        for _ in 0..index {
            current_node = match current_node {
                Some(node) => &node.next,
                None => return None,
            };
        }
        current_node.as_ref().map(|node| &node.data)
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn push_at(&mut self, index: usize, data: T) {
        let mut current_node = &mut self.head;

        for _ in 0..index {
            current_node = match current_node {
                Some(node) => &mut node.next,
                None => return,
            };
        }

        let new_node = Box::new(Node {
            data,
            next: current_node.take(),
        });

        *current_node = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop();
        }

        let mut current_node = &mut self.head;

        for _ in 0..index - 1 {
            current_node = match current_node {
                Some(node) => &mut node.next,
                None => return None,
            };
        }

        let mut target_node = current_node.as_mut()?.next.take()?;
        let next_node = target_node.next.take(); // Take out the next node first

        if let Some(node) = current_node.as_mut() {
            node.next = next_node
        };

        Some(target_node.data) // Return the taken data
    }
}

impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for MyLinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    fn new_list_is_empty() {
        let list: MyLinkedList<i32> = MyLinkedList::new();
        assert!(list.peek().is_none());
    }

    #[test]
    fn push_and_pop_with_integers() {
        let mut list = MyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_and_peek_at_with_strings() {
        let mut list = MyLinkedList::new();
        list.push("apple".to_string());
        list.push("banana".to_string());
        assert_eq!(list.peek(), Some(&"banana".to_string()));
        assert_eq!(list.peek_at(1), Some(&"apple".to_string()));
    }

    #[test]
    fn push_at_with_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut list = MyLinkedList::new();
        list.push(Point { x: 1, y: 2 });
        list.push_at(1, Point { x: 3, y: 4 });
        assert_eq!(list.pop_at(1), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.pop(), Some(Point { x: 1, y: 2 }));
    }

    #[test]
    fn peek_at_out_of_bounds_returns_none() {
        let mut list = MyLinkedList::new();
        list.push(10);
        list.push(20);
        assert_eq!(list.peek_at(2), None);
    }

    #[test]
    fn pop_at_from_empty_list() {
        let mut list: MyLinkedList<f64> = MyLinkedList::new();
        assert_eq!(list.pop_at(0), None);
    }

    #[test]
    fn push_and_peek_with_struct() {
        #[derive(Debug, PartialEq)]
        struct TestStruct {
            a: i32,
            b: char,
        }

        let mut list = MyLinkedList::new();
        let item = TestStruct { a: 1, b: 'a' };
        list.push(item);

        assert_eq!(list.peek(), Some(&TestStruct { a: 1, b: 'a' }));
    }

    #[test]
    fn push_at_start_and_middle() {
        let mut list = MyLinkedList::new();
        list.push_at(0, "start");
        list.push("middle");
        list.push_at(1, "end");

        assert_eq!(list.pop(), Some("middle"));
        assert_eq!(list.pop(), Some("end"));
        assert_eq!(list.pop(), Some("start"));
    }

    #[test]
    fn complex_sequence_of_operations() {
        let mut list = MyLinkedList::new();
        list.push(3); // List: [3]
        list.push(1); // List: [1, 3]
        list.push_at(1, 2); // List: [1, 2, 3]
        assert_eq!(list.pop(), Some(1)); // List: [2, 3]
        list.push_at(1, 4); // List: [2, 4, 3]

        assert_eq!(list.peek(), Some(&2)); // Peek at first element
        assert_eq!(list.peek_at(1), Some(&4)); // Peek at second element
        assert_eq!(list.pop_at(2), Some(3)); // Remove third element
    }

    #[test]
    fn iterator_test() {
        let mut list = MyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut elements = Vec::new();
        for item in &mut list {
            elements.push(item);
        }

        assert_eq!(elements, vec![3, 2, 1]);
    }
}

// reference: https://rust-unofficial.github.io/too-many-lists/second-peek.html
