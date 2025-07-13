pub struct CharacterDisplay {
    capacity: usize,
    
    buffer: String,
    
    data: String,
    data_updated: bool
}

impl CharacterDisplay {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            
            buffer: String::with_capacity(capacity),
            
            data: String::with_capacity(capacity),
            data_updated: true
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
        self.data_updated = true;
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
        
        self.data.clear();
        self.data_updated = true;
    }

    pub fn data_updated(&self) -> bool {
        self.data_updated
    }

    pub fn disable_data_updated(&mut self) {
        self.data_updated = false;
    }
}