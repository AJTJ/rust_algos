#[derive(Debug)]
pub struct BinaryNode {
    val: i32,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

#[derive(Debug)]
pub struct BinaryTree {
    root: Box<BinaryNode>,
}

impl BinaryTree {
    pub fn new(node: BinaryNode) -> BinaryTree {
        BinaryTree {
            root: Box::new(node),
        }
    }

    pub fn fill_ordered(&mut self, val: i32) {
        let mut current = Some(&mut self.root);

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
                    current = cur_box.right.as_mut();
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
                    current = cur_box.left.as_mut();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let root_node = BinaryNode {
            val: 5,
            left: None,
            right: None,
        };

        let mut my_tree = BinaryTree::new(root_node);

        vec![3, 6, 2, 7, 4]
            .iter()
            .for_each(|x| my_tree.fill_ordered(*x));

        println!("my tree root: {:?}", my_tree.root);
    }
}
