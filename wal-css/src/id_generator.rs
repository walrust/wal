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

#[cfg(test)]
mod tests {
    use web_sys::Element;

    use super::IdGenerator;

    #[test]
    fn generator_generates_new_ids() {
        let mut gen = IdGenerator::new();

        assert_eq!(0, gen.get_new_id());
        assert_eq!(1, gen.get_new_id());
        assert_eq!(2, gen.get_new_id());
    }
}
