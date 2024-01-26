use std::clone::Clone;

#[derive(PartialEq, Clone)]
pub struct TreeNode<T>
where
    T: Clone,
{
    data: T,
    left_child: Option<Box<TreeNode<T>>>,
    right_child: Option<Box<TreeNode<T>>>,
}

pub struct BinarySearchTree<T: Clone> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
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
                    let successor_parent = target.clone();
                    let target_data = target.data;

                    match (target.left_child, target.right_child) {
                        (None, None) => Some(target_data),
                        (Some(left_child), None) | (None, Some(left_child)) => {
                            parent_node.left_child = Some(left_child);
                            Some(target_data)
                        }
                        (Some(_left_child), Some(right_child)) => {
                            let successor_node =
                                Self::find_successor_node(*successor_parent, right_child);

                            parent_node.left_child = Some(successor_node);
                            Some(target_data)
                        }
                    }
                } else {
                    let target = parent_node.right_child.take()?;
                    let successor_parent = target.clone();
                    let target_data = target.data;

                    match (target.left_child, target.right_child) {
                        (None, None) => Some(target_data),
                        (Some(right_child), None) | (None, Some(right_child)) => {
                            parent_node.right_child = Some(right_child);
                            Some(target_data)
                        }
                        (Some(_left_child), Some(right_child)) => {
                            let successor_node =
                                Self::find_successor_node(*successor_parent, right_child);

                            parent_node.right_child = Some(successor_node);
                            Some(target_data)
                        }
                    }
                }
            }
        }
    }

    fn find_successor_node(
        successor_parent: TreeNode<T>,
        mut new_target: Box<TreeNode<T>>,
    ) -> Box<TreeNode<T>> {
        if new_target.left_child.is_none() {
            new_target.left_child = successor_parent.left_child;
            return new_target;
        }

        let mut new_target_child = new_target.left_child.as_ref().unwrap();

        loop {
            if new_target_child.left_child.is_none() {
                new_target.left_child.as_mut().unwrap().left_child = successor_parent.left_child;

                if let Some(right_child) = new_target.right_child.take() {
                    new_target.right_child = Some(right_child);
                }
                return new_target;
            } else {
                new_target_child = &new_target_child.left_child.as_ref().unwrap();
            }
        }
    }
}

impl<T: PartialOrd + Clone> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;
    use super::TreeNode;

    #[test]
    fn insert_and_search() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);
        bst.insert(8);
        bst.insert(12);

        assert!(bst.search(15).is_some());
        assert!(bst.search(10).is_some());
        assert!(bst.search(20).is_some());
        assert!(bst.search(8).is_some());
        assert!(bst.search(12).is_some());
        assert!(bst.search(100).is_none());
    }

    #[test]
    fn remove_leaf_node() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);

        assert_eq!(bst.remove(20), Some(20));
        assert!(bst.search(20).is_none());
    }

    #[test]
    fn remove_node_with_one_child() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);
        bst.insert(25);

        assert_eq!(bst.remove(20), Some(20));
        assert!(bst.search(20).is_none());
        assert!(bst.search(25).is_some());
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);
        bst.insert(17);
        bst.insert(25);

        assert_eq!(bst.remove(20), Some(20));

        assert!(bst.search(20).is_none());
        assert!(bst.search(17).is_some());
        assert!(bst.search(25).is_some());
    }

    #[test]
    fn nonexistent_element_removal_and_search() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);

        assert_eq!(bst.remove(100), None);
        assert!(bst.search(100).is_none());
    }

    impl<T: PartialOrd + Clone> BinarySearchTree<T> {
        // Helper method to perform in-order traversal
        pub fn in_order_traversal(&self) -> Vec<T> {
            fn in_order<T: PartialOrd + Clone>(
                node: &Option<Box<TreeNode<T>>>,
                values: &mut Vec<T>,
            ) {
                if let Some(node) = node {
                    in_order(&node.left_child, values);
                    values.push(node.data.clone());
                    in_order(&node.right_child, values);
                }
            }

            let mut values = Vec::new();
            in_order(&self.root, &mut values);
            values
        }
    }

    #[test]
    fn in_order_traversal() {
        let mut bst = BinarySearchTree::new();
        bst.insert(15);
        bst.insert(10);
        bst.insert(20);
        bst.insert(5);
        bst.insert(12);

        let traversal = bst.in_order_traversal();
        assert_eq!(traversal, vec![5, 10, 12, 15, 20]);
    }

    // TODO: Resolve lasting problem of data the structure: remove root capability
    // #[test]
    // fn remove_root() {
    //     let mut bst = BinarySearchTree::new();
    //     bst.insert(15);
    //     bst.insert(10);
    //     bst.insert(20);
    //     bst.insert(25);

    //     assert_eq!(bst.remove(15), Some(15));
    //     assert!(bst.search(15).is_none());
    //     assert!(bst.search(10).is_some());
    //     assert!(bst.search(20).is_some());
    //     assert!(bst.search(25).is_some());

    //     // Optionally check the new root if necessary
    //     // assert_eq!(bst.root.as_ref().unwrap().data, <expected new root value>);
    // }
}
