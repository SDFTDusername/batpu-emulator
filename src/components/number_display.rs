pub struct NumberDisplay {
    value: u8,
    pub signed: bool
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
    
    pub fn clear(&mut self) {
        self.value = 0;
    }
}