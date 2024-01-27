pub struct Queue<T> {
    vec: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { vec: Vec::new() }
    }

    pub fn enqueue(&mut self, element: T) {
        self.vec.push(element)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            None
        } else {
            Some(self.vec.remove(0))
        }
    }

    pub fn read(&self) -> Option<&T> {
        self.vec.first()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn enqueue_and_read() {
        let mut queue = Queue::new();
        assert!(queue.read().is_none());

        queue.enqueue(1);
        assert_eq!(*queue.read().unwrap(), 1);

        queue.enqueue(2);
        assert_eq!(*queue.read().unwrap(), 1);
    }

    #[test]
    fn enqueue_and_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn queue_with_different_types() {
        let mut string_queue = Queue::new();
        string_queue.enqueue("hello");
        string_queue.enqueue("world");

        assert_eq!(string_queue.dequeue(), Some("hello"));
        assert_eq!(string_queue.dequeue(), Some("world"));

        let mut int_queue = Queue::new();
        int_queue.enqueue(42);
        int_queue.enqueue(100);

        assert_eq!(int_queue.dequeue(), Some(42));
        assert_eq!(int_queue.dequeue(), Some(100));
    }

    #[test]
    fn dequeue_empty_queue() {
        let mut queue = Queue::<i32>::new();
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn multiple_enqueue_dequeue_operations() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        assert_eq!(queue.dequeue(), Some(10));

        queue.enqueue(30);
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn read_after_dequeue_operations() {
        let mut queue = Queue::new();
        queue.enqueue("a");
        queue.enqueue("b");
        queue.dequeue();

        assert_eq!(queue.read(), Some(&"b"));
        queue.dequeue();
        assert_eq!(queue.read(), None);
    }

    #[test]
    fn default_constructor() {
        let queue: Queue<u32> = Queue::default();
        assert!(queue.read().is_none());
    }

    #[test]
    fn queue_with_complex_type() {
        let mut queue = Queue::new();
        queue.enqueue(vec![1, 2, 3]);
        queue.enqueue(vec![4, 5, 6]);

        assert_eq!(queue.dequeue(), Some(vec![1, 2, 3]));
        assert_eq!(queue.dequeue(), Some(vec![4, 5, 6]));
        assert_eq!(queue.dequeue(), None);
    }
}
