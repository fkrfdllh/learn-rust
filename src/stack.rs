pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(items: Vec<T>) -> Self {
        return Stack { items };
    }

    pub fn get(&mut self) -> Option<T> {
        return self.items.pop();
    }

    pub fn put(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn is_empty(&self) -> bool {
        return self.items.is_empty();
    }
}
