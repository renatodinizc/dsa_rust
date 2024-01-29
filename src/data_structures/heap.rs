pub struct Heap<T> {
    vec: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap { vec: Vec::new() }
    }

    pub fn peek(&self) -> Option<&T> {
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

    fn parent_index(index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }

        let size = self.vec.len();
        self.vec.swap(0, size - 1); // Swap the root with the last element
        let result = self.vec.pop(); // Remove the last element (original root)

        let mut current = 0;
        loop {
            let left = Self::left_child_index(current);
            let right = Self::right_child_index(current);
            let mut largest = current;

            if left < size - 1 && self.vec[left] > self.vec[largest] {
                largest = left;
            }

            if right < size - 1 && self.vec[right] > self.vec[largest] {
                largest = right;
            }

            if largest != current {
                self.vec.swap(current, largest);
                current = largest;
            } else {
                break;
            }
        }

        result
    }

    fn left_child_index(index: usize) -> usize {
        index * 2 + 1
    }

    fn right_child_index(index: usize) -> usize {
        index * 2 + 2
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
        assert!(heap.peek().is_none());
    }

    #[test]
    fn insertion() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(5);
        assert_eq!(heap.peek(), Some(&20));
    }

    #[test]
    fn removal() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(15);
        assert_eq!(heap.remove(), Some(20));
        assert_eq!(heap.peek(), Some(&15));
    }

    #[test]
    fn with_different_types() {
        let mut float_heap = Heap::new();
        float_heap.insert(1.5);
        float_heap.insert(2.5);
        assert_eq!(float_heap.peek(), Some(&2.5));

        let mut int_heap = Heap::new();
        int_heap.insert(1);
        int_heap.insert(2);
        assert_eq!(int_heap.peek(), Some(&2));
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
        assert_eq!(heap.peek(), Some(&"cherry".to_string()));
    }

    #[test]
    fn removal_with_strings() {
        let mut heap = Heap::new();
        heap.insert("apple".to_string());
        heap.insert("banana".to_string());
        heap.insert("cherry".to_string());
        assert_eq!(heap.remove(), Some("cherry".to_string()));
        assert_eq!(heap.peek(), Some(&"banana".to_string()));
    }

    #[test]
    fn insertion_with_str() {
        let mut heap = Heap::new();
        heap.insert("apple");
        heap.insert("banana");
        heap.insert("cherry");
        assert_eq!(heap.peek(), Some(&"cherry"));
    }

    #[test]
    fn removal_with_str() {
        let mut heap = Heap::new();
        heap.insert("apple");
        heap.insert("banana");
        heap.insert("cherry");
        assert_eq!(heap.remove(), Some("cherry"));
        assert_eq!(heap.peek(), Some(&"banana"));
    }

    #[test]
    fn test_heap_order() {
        let mut heap = Heap::new();
        heap.insert(55);
        heap.insert(22);
        heap.insert(34);
        heap.insert(10);
        heap.insert(2);
        heap.insert(99);
        heap.insert(68);

        let mut result = Vec::new();
        while let Some(number) = heap.remove() {
            result.push(number);
        }

        assert_eq!(result, vec![99, 68, 55, 34, 22, 10, 2]);
    }
}
