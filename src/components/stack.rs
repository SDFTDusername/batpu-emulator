use batpu_assembly::components::address;

pub struct Stack {
    max_size: u32,
    
    stack: Vec<u32>,
    stack_updated: bool
}

impl Stack {
    pub fn new(max_size: u32) -> Self {
        Self {
            max_size,
            
            stack: Vec::with_capacity(max_size as usize),
            stack_updated: true
        }
    }
    
    pub fn push(&mut self, address: u32) -> bool {
        if address > address::MAX_VALUE {
            panic!("Address {} out of range, expected 0-{}", address, address::MAX_VALUE);
        }
        
        if self.stack.len() as u32 == self.max_size {
            self.stack.remove(0);
        }
        
        self.stack.push(address);
        self.stack_updated = true;
        
        true
    }
    
    pub fn pop(&mut self) -> u32 {
        let result = self.stack.pop();
        match result {
            Some(value) => {
                self.stack_updated = true;
                value
            },
            None => 0
        }
    }
    
    pub fn clear(&mut self) {
        self.stack.clear();
        self.stack_updated = true;
    }
    
    pub fn stack(&self) -> &[u32] {
        &self.stack
    }

    pub fn stack_updated(&self) -> bool {
        self.stack_updated
    }

    pub fn disable_stack_updated(&mut self) {
        self.stack_updated = false;
    }
}