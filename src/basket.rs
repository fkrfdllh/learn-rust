pub struct Basket {
    item: Option<String>,
}

impl Basket {
    pub fn new(item: String) -> Self {
        return Basket { item: Some(item) };
    }

    pub fn get(&mut self) -> Option<String> {
        return self.item.take();
    }

    pub fn put(&mut self, item: String) {
        self.item = Some(item);
    }

    pub fn is_empty(&self) -> bool {
        return self.item.is_none();
    }
}
