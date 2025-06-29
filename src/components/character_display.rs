pub struct CharacterDisplay {
    capacity: usize,
    
    buffer: String,
    data: String
}

impl CharacterDisplay {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            
            buffer: String::with_capacity(capacity),
            data: String::with_capacity(capacity)
        }
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn push(&mut self, character: Option<&char>) -> bool {
        if self.buffer.len() == self.capacity {
            return false;
        }
        
        match character {
            Some(&character) => {
                self.buffer.push(character);
                true
            },
            None => false
        }
    }
    
    pub fn data(&self) -> &str {
        &self.data
    }
    
    pub fn push_buffer(&mut self) {
        self.data.clone_from(&self.buffer);
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.data.clear();
    }
}