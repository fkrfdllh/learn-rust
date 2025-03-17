pub struct Basket<T> {
    item: Option<T>,
}

impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        return Basket { item: Some(item) };
    }

    pub fn get(&mut self) -> Option<T> {
        return self.item.take();
    }

    pub fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    pub fn is_empty(&self) -> bool {
        return self.item.is_none();
    }
}
