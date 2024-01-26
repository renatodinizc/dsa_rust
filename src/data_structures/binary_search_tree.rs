#[derive(Debug, PartialEq, Clone)]
pub struct TreeNode<T> {
    data: T,
    left_child: Option<Box<TreeNode<T>>>,
    right_child: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn search(&self, data: T) -> Option<&TreeNode<T>> {
        match self.root.as_ref() {
            None => None,
            Some(mut node) => loop {
                if data == node.data {
                    return Some(node);
                } else if data < node.data {
                    if node.left_child.is_none() {
                        return None;
                    } else {
                        node = node.left_child.as_ref().unwrap();
                    }
                } else {
                    if node.right_child.is_none() {
                        return None;
                    } else {
                        node = node.right_child.as_ref().unwrap();
                    }
                }
            },
        }
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(TreeNode {
            data,
            left_child: None,
            right_child: None,
        });

        match self.root.as_mut() {
            None => self.root = Some(new_node),
            Some(mut current_node) => loop {
                if new_node.data < current_node.data {
                    if current_node.left_child.is_none() {
                        current_node.left_child = Some(new_node);
                        break;
                    } else {
                        current_node = current_node.left_child.as_mut().unwrap();
                    }
                } else {
                    if current_node.right_child.is_none() {
                        current_node.right_child = Some(new_node);
                        break;
                    } else {
                        current_node = current_node.right_child.as_mut().unwrap();
                    }
                }
            },
        }
    }

    pub fn remove(&mut self, data: T) -> Option<T> {
        let pair = match self.root.as_mut() {
            None => None,
            Some(mut node) => loop {
                if data < node.data {
                    if node.left_child.is_none() {
                        break None;
                    } else if node.left_child.as_mut().unwrap().data == data {
                        break Some((node, "left"));
                    } else {
                        node = node.left_child.as_mut().unwrap();
                    }
                } else {
                    if node.right_child.is_none() {
                        break None;
                    } else if node.right_child.as_ref().unwrap().data == data {
                        break Some((node, "right"));
                    } else {
                        node = node.right_child.as_mut().unwrap();
                    }
                }
            },
        };

        match pair {
            None => None,
            Some((parent_node, side)) => {
                if side == "left" {
                    let target = parent_node.left_child.take()?;
                    let target_data = target.data;

                    match (target.left_child, target.right_child) {
                        (None, None) => Some(target_data),
                        (Some(left_child), None) | (None, Some(left_child)) => {
                            parent_node.left_child = Some(left_child);
                            Some(target_data)
                        }
                        (Some(_left_child), Some(_right_child)) => panic!("asd"),
                    }
                } else {
                    let target = parent_node.right_child.take()?;
                    let target_data = target.data;

                    match (target.left_child, target.right_child) {
                        (None, None) => Some(target_data),
                        (Some(right_child), None) | (None, Some(right_child)) => {
                            parent_node.right_child = Some(right_child);
                            Some(target_data)
                        }
                        (Some(_left_child), Some(_right_child)) => panic!("asd"),
                    }
                }
            }
        }
    }
}

impl<T: PartialOrd> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_binary_search_tree() {
    let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();

    bst.insert(50);
    bst.insert(75);
    bst.insert(25);
    bst.insert(33);
    bst.insert(56);
    bst.insert(89);
    bst.insert(10);

    bst.insert(40);
    bst.insert(52);
    bst.insert(61);
    bst.insert(95);
    bst.insert(4);
    bst.insert(82);
    bst.insert(11);
    bst.insert(30);

    bst.insert(45);

    // assert_eq!(bst.search(45).unwrap().data, 45);
    // assert_eq!(bst.search(99), None);

    // assert_eq!(bst.remove(30), Some(30));
    // assert_eq!(bst.remove(99), None);
    assert_eq!(bst.remove(40), Some(40));
    println!("{:#?}", bst);
}
