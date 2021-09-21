
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<T> {
        self.stack.last()
    }
}

fn setup() {
    let mut stack: Stack<isize> = Stack::new();
    stack.push(1);
    let item = stack.pop();
    assert_eq!(item.unwrap(), 1);
}
