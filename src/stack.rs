pub struct Stack {
    max_size: usize,
    stack: Vec<usize>
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            stack: Vec::with_capacity(max_size)
        }
    }
    
    pub fn push(&mut self, address: usize) -> bool {
        if address > 1023 {
            panic!("Address {} out of range, expected 0-1023", address);
        }
        
        if self.stack.len() == self.max_size {
            return false;
        }
        
        self.stack.push(address);
        true
    }
    
    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
    
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}