pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { vec: Vec::new() }
    }

    pub fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn read(&self) -> Option<&T> {
        self.vec.last()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Stack<T> {
        Stack::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn push_and_read() {
        let mut stack = Stack::new();
        assert!(stack.read().is_none());

        stack.push(1);
        assert_eq!(*stack.read().unwrap(), 1);

        stack.push(2);
        assert_eq!(*stack.read().unwrap(), 2);
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn read_empty_stack() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.read().is_none());
    }

    #[test]
    fn stack_with_string() {
        let mut stack = Stack::new();
        stack.push("Hello".to_string());
        stack.push("World".to_string());

        assert_eq!(stack.pop(), Some("World".to_string()));
        assert_eq!(stack.pop(), Some("Hello".to_string()));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_with_str_slice() {
        let mut stack = Stack::new();
        stack.push("Apple");
        stack.push("Banana");

        assert_eq!(stack.pop(), Some("Banana"));
        assert_eq!(stack.pop(), Some("Apple"));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_with_vector() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        stack.push(vec![4, 5, 6]);

        assert_eq!(stack.pop(), Some(vec![4, 5, 6]));
        assert_eq!(stack.pop(), Some(vec![1, 2, 3]));
        assert_eq!(stack.pop(), None);
    }
}
