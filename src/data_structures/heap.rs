pub struct Heap<T> {
    vec: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap { vec: Vec::new() }
    }

    pub fn first_node(&self) -> Option<&T> {
        self.vec.first()
    }

    pub fn insert(&mut self, value: T) {
        self.vec.push(value);
        let mut new_node_index = self.vec.len() - 1;

        loop {
            if new_node_index == 0 {
                break;
            } else if self.vec[new_node_index] > self.vec[Self::parent_index(new_node_index)] {
                self.vec
                    .swap(new_node_index, Self::parent_index(new_node_index));
                new_node_index = Self::parent_index(new_node_index);
            } else {
                break;
            }
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }
        let last_index = self.vec.len() - 1;

        self.vec.swap(0, last_index);

        let mut current_node_index = 0;

        loop {
            match (
                Self::left_child_value(self, current_node_index),
                Self::right_child_value(self, current_node_index),
            ) {
                (Some(value), None) => {
                    if self.vec[current_node_index] > *value {
                        self.vec.swap(
                            current_node_index,
                            Self::left_child_index(current_node_index),
                        )
                    };
                    current_node_index = Self::left_child_index(current_node_index);
                }
                (None, Some(value)) => {
                    if self.vec[current_node_index] > *value {
                        self.vec.swap(
                            current_node_index,
                            Self::right_child_index(current_node_index),
                        )
                    };
                    current_node_index = Self::right_child_index(current_node_index);
                }
                (Some(left_value), Some(right_value)) => {
                    if left_value > right_value {
                        if self.vec[current_node_index] > *left_value {
                            self.vec.swap(
                                current_node_index,
                                Self::left_child_index(current_node_index),
                            )
                        };
                        current_node_index = Self::left_child_index(current_node_index);
                    } else {
                        if self.vec[current_node_index] > *right_value {
                            self.vec.swap(
                                current_node_index,
                                Self::right_child_index(current_node_index),
                            )
                        };
                        current_node_index = Self::right_child_index(current_node_index);
                    }
                }
                (None, None) => break,
            }
        }

        self.vec.pop()
    }

    fn left_child_index(index: usize) -> usize {
        index * 2 + 1
    }

    fn left_child_value(&self, index: usize) -> Option<&T> {
        self.vec.get(Self::left_child_index(index))
    }

    fn right_child_index(index: usize) -> usize {
        index * 2 + 2
    }

    fn right_child_value(&self, index: usize) -> Option<&T> {
        self.vec.get(Self::right_child_index(index))
    }

    fn parent_index(index: usize) -> usize {
        (index - 1) / 2
    }
}

impl<T: PartialOrd> Default for Heap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let heap: Heap<i32> = Heap::new();
        assert!(heap.first_node().is_none());
    }

    #[test]
    fn insertion() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(5);
        assert_eq!(heap.first_node(), Some(&20));
    }

    #[test]
    fn removal() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(15);
        assert_eq!(heap.remove(), Some(20));
        assert_eq!(heap.first_node(), Some(&15));
    }

    #[test]
    fn with_different_types() {
        let mut float_heap = Heap::new();
        float_heap.insert(1.5);
        float_heap.insert(2.5);
        assert_eq!(float_heap.first_node(), Some(&2.5));

        let mut int_heap = Heap::new();
        int_heap.insert(1);
        int_heap.insert(2);
        assert_eq!(int_heap.first_node(), Some(&2));
    }

    #[test]
    fn empty_removal() {
        let mut heap: Heap<i32> = Heap::new();
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn order_maintenance() {
        let mut heap = Heap::new();
        heap.insert(3);
        heap.insert(1);
        heap.insert(4);
        heap.insert(1);
        heap.insert(5);
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), Some(4));
        assert_eq!(heap.remove(), Some(3));
        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), None);
    }

    #[test]
    fn insertion_with_strings() {
        let mut heap = Heap::new();
        heap.insert("apple".to_string());
        heap.insert("banana".to_string());
        heap.insert("cherry".to_string());
        assert_eq!(heap.first_node(), Some(&"cherry".to_string()));
    }

    #[test]
    fn removal_with_strings() {
        let mut heap = Heap::new();
        heap.insert("apple".to_string());
        heap.insert("banana".to_string());
        heap.insert("cherry".to_string());
        assert_eq!(heap.remove(), Some("cherry".to_string()));
        assert_eq!(heap.first_node(), Some(&"banana".to_string()));
    }

    #[test]
    fn insertion_with_str() {
        let mut heap = Heap::new();
        heap.insert("apple");
        heap.insert("banana");
        heap.insert("cherry");
        assert_eq!(heap.first_node(), Some(&"cherry"));
    }

    #[test]
    fn removal_with_str() {
        let mut heap = Heap::new();
        heap.insert("apple");
        heap.insert("banana");
        heap.insert("cherry");
        assert_eq!(heap.remove(), Some("cherry"));
        assert_eq!(heap.first_node(), Some(&"banana"));
    }
}
