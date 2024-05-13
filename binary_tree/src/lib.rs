#[derive(Debug, Clone)]
pub struct BinaryNode {
    val: i32,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

#[derive(Debug, Clone)]
pub struct BinaryTree {
    root: Option<Box<BinaryNode>>,
}

impl BinaryTree {
    pub fn new(node: Option<BinaryNode>) -> BinaryTree {
        BinaryTree {
            root: node.map(|n| Box::new(n)),
        }
    }

    pub fn fill_ordered(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(BinaryNode {
                val,
                left: None,
                right: None,
            }));
        } else {
            let mut current = &mut self.root;

            while let Some(cur_box) = current {
                if val > cur_box.val {
                    if cur_box.right.is_none() {
                        cur_box.right = Some(Box::new(BinaryNode {
                            val,
                            left: None,
                            right: None,
                        }));
                        break;
                    } else {
                        current = &mut cur_box.right;
                    }
                } else {
                    if cur_box.left.is_none() {
                        cur_box.left = Some(Box::new(BinaryNode {
                            val,
                            left: None,
                            right: None,
                        }));
                        break;
                    } else {
                        current = &mut cur_box.left;
                    }
                }
            }
        }
    }

    pub fn in_order_traversal(&self) -> Vec<i32> {
        let mut return_val = vec![];

        fn traverse(n: &Option<Box<BinaryNode>>, v: &mut Vec<i32>) {
            if let Some(node) = n {
                traverse(&node.left, v);
                v.push(node.val);
                traverse(&node.right, v);
            }
        }

        traverse(&self.root, &mut return_val);

        return_val
    }
    pub fn pre_order_traversal(&self) -> Vec<i32> {
        let mut return_val = vec![];

        fn traverse(n: &Option<Box<BinaryNode>>, v: &mut Vec<i32>) {
            if let Some(node) = n {
                v.push(node.val);
                traverse(&node.left, v);
                traverse(&node.right, v);
            }
        }

        traverse(&self.root, &mut return_val);

        return_val
    }
    pub fn post_order_traversal(&self) -> Vec<i32> {
        let mut return_val = vec![];

        fn traverse(n: &Option<Box<BinaryNode>>, v: &mut Vec<i32>) {
            if let Some(node) = n {
                traverse(&node.left, v);
                traverse(&node.right, v);
                v.push(node.val);
            }
        }

        traverse(&self.root, &mut return_val);

        return_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut my_tree = BinaryTree::new(None);
        let test_val = vec![5, 3, 6, 2, 7, 4];

        test_val.iter().for_each(|x| my_tree.fill_ordered(*x));

        println!("my tree root: {:?}", my_tree.root);
        assert_eq!(my_tree.clone().root.unwrap().val, 5);
        assert_eq!(
            &my_tree.root.clone().unwrap().left.as_ref().unwrap().val,
            &3
        );

        let mut sorted_test_vals = test_val.clone();
        sorted_test_vals.sort();
        let in_order = my_tree.in_order_traversal();
        let pre_order = my_tree.pre_order_traversal();
        let post_order = my_tree.post_order_traversal();
        println!("in_order: {in_order:?}");
        println!("pre_order: {pre_order:?}");
        println!("post_order: {post_order:?}");
        assert_eq!(in_order, sorted_test_vals);
    }
}
