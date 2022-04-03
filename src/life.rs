pub struct Life {
    pub(crate) generation: i32,
}

impl Life {
    pub fn increment(&mut self) {
        self.generation += 1
    }

    pub fn is_generation_even(&self) -> bool {
        self.generation % 2 == 0
    }
}