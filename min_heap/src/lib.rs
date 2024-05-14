#[derive(Debug, Clone)]
pub struct MinHeapNode {
    value: i32,
    left: Option<Box<MinHeapNode>>,
    right: Option<Box<MinHeapNode>>,
}

#[derive(Debug, Clone)]
pub struct MinHeap {
    root: Option<Box<MinHeapNode>>,
}

impl MinHeap {
    pub fn insert(&mut self, value: i32) {
        let new_node = MinHeapNode {
            value,
            left: None,
            right: None,
        };

        // place the new node at the first empty leaf node at the bottom right

        let mut queue: Vec<&mut Box<MinHeapNode>> = vec![];
        let mut current_node: Option<&mut Box<MinHeapNode>> = None;
        let mut current_index: usize = 0;

        if let Some(root) = &mut self.root {
            queue.push(root);
            // getting lots of errors with queue, since I'm borring it mutably a lot

            let q_len = queue.len();
            while q_len > 0 {
                current_node = Some(&mut queue[current_index]);
                if let Some(cur) = current_node {
                    if cur.left.is_none() {
                        cur.left = Some(Box::new(new_node));
                        break;
                    } else if cur.right.is_none() {
                        cur.right = Some(Box::new(new_node));
                        break;
                    } else {
                        queue.push(cur.left.as_mut().unwrap());
                        queue.push(cur.right.as_mut().unwrap());
                        current_index += 1;
                    }
                }
            }
        } else {
            self.root = Some(Box::new(new_node));
            return;
        }

        // NEXT: bubble up the value of this new leaf node

        let parent_node = current_node;

        //     let parentNode = currentNode;
        // currentNode = newNode;
        // while (parentNode && parentNode?.value > currentNode.value) {
        //   // switch the values
        //   // NOTE: this is the stuff that is somewhat complex
        //   const temp = parentNode.value;
        //   parentNode.value = currentNode.value;
        //   currentNode.value = temp;

        //   currentIndex = Math.floor(currentIndex / 2);
        //   const tempNode = parentNode;
        //   parentNode = queue[currentIndex];
        //   currentNode = tempNode;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
