// A Stack

struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);
}
