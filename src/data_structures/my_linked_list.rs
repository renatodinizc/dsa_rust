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

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
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
}

impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    fn new_list_should_be_empty() {
        let mut list: MyLinkedList<i32> = MyLinkedList::new();
        assert!(list.pop().is_none());
    }

    #[test]
    fn push_should_add_elements() {
        let mut list = MyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn pop_should_return_none_on_empty_list() {
        let mut list: MyLinkedList<i32> = MyLinkedList::new();
        assert!(list.pop().is_none());
    }

    #[test]
    fn push_and_pop_combination() {
        let mut list = MyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert!(list.pop().is_none());
    }

    #[test]
    fn push_and_pop_with_string() {
        let mut list = MyLinkedList::new();
        list.push("hello".to_string());
        list.push("world".to_string());
        assert_eq!(list.pop(), Some("world".to_string()));
        assert_eq!(list.pop(), Some("hello".to_string()));
    }

    #[test]
    fn push_and_pop_with_custom_struct() {
        let mut list = MyLinkedList::new();

        #[derive(PartialEq, Debug)]
        struct TestStruct {
            field1: i32,
            field2: String,
        }

        let item1 = TestStruct {
            field1: 1,
            field2: "first".to_string(),
        };
        let item2 = TestStruct {
            field1: 2,
            field2: "second".to_string(),
        };

        list.push(item1);
        list.push(item2);

        assert_eq!(
            list.pop(),
            Some(TestStruct {
                field1: 2,
                field2: "second".to_string()
            })
        );
        assert_eq!(
            list.pop(),
            Some(TestStruct {
                field1: 1,
                field2: "first".to_string()
            })
        );
    }
}
