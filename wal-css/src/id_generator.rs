pub struct IdGenerator {
    next_id: u8,
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator { next_id: 0 }
    }

    pub fn get_new_id(&mut self) -> u8 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Add tests
