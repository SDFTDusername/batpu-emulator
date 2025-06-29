pub struct NumberDisplay {
    value: u8,
    signed: bool
}

impl NumberDisplay {
    pub fn new() -> Self {
        Self {
            value: 0,
            signed: false
        }
    }

    pub fn value(&self) -> isize {
        if self.signed {
            self.value as isize - 128
        } else {
            self.value as isize
        }
    }
    
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
    
    pub fn show(&mut self, value: u8) {
        self.value = value;
    }
    
    pub fn clear(&mut self) {
        self.value = 0;
    }
    
    pub fn signed(&self) -> bool {
        self.signed
    }
    
    pub fn set_signed(&mut self, signed: bool) {
        self.signed = signed;
    }
}