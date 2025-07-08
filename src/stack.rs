use batpu_assembly::components::address;

pub struct Stack {
    max_size: u32,
    stack: Vec<u32>
}

impl Stack {
    pub fn new(max_size: u32) -> Self {
        Self {
            max_size,
            stack: Vec::with_capacity(max_size as usize)
        }
    }
    
    pub fn push(&mut self, address: u32) -> bool {
        if address > address::MAX_VALUE {
            panic!("Address {} out of range, expected 0-{}", address, address::MAX_VALUE);
        }
        
        if self.stack.len() as u32 == self.max_size {
            return false;
        }
        
        self.stack.push(address);
        true
    }
    
    pub fn pop(&mut self) -> Option<u32> {
        self.stack.pop()
    }
    
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    pub fn stack(&self) -> &[u32] {
        &self.stack
    }
}