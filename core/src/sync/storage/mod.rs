#[derive(Debug)]

pub struct SyncQueue {
    items: Vec<Vec<u8>>,
}

impl SyncQueue {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, data: Vec<u8>) {
        self.items.push(data);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}
