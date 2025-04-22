#[derive(Debug)]
pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            stack: Vec::<T>::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.stack.push(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}
