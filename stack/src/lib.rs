// TODO: use traits

// STACKS
// LIFO Last In First Out
// QUEUES
// FIFO First In First Out

struct StackNode<T> {
    // TODO make generic or enum?
    value: T,
    next: Option<Box<StackNode<T>>>,
}

pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T: Clone> Stack<T> {
    pub fn create() -> Stack<T> {
        Stack { top: None }
    }

    pub fn is_empty(&self) -> bool {
        !self.top.is_some()
    }

    pub fn peek(&self) -> Option<T> {
        self.top.as_ref().map(|x| x.value.clone())
    }

    pub fn push(&mut self, val: T) {
        let node = StackNode {
            value: val,
            next: self.top.take(),
        };

        self.top = Some(Box::new(node))
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(top_val) = &mut self.top {
            let data = top_val.value.clone();
            self.top = top_val.next.take();
            Some(data)
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Stack;

    #[test]
    fn it_works() {
        let mut my_stack = Stack::create();

        assert_eq!(my_stack.is_empty(), true);

        my_stack.push(1);

        assert_eq!(my_stack.is_empty(), false);

        my_stack.push(2);
        my_stack.push(3);

        assert_eq!(my_stack.peek().unwrap(), 3);

        my_stack.pop();

        assert_eq!(my_stack.peek().unwrap(), 2);
    }
}
