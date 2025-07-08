use crate::machine::Word;

pub struct NumberDisplay {
    value: Word,
    pub signed: bool
}

impl NumberDisplay {
    pub fn new() -> Self {
        Self {
            value: 0,
            signed: false
        }
    }

    pub fn value(&self) -> i32 {
        if self.signed {
            ((self.value as i32) << (32 - Word::BITS)) >> (32 - Word::BITS)
        } else {
            self.value as i32
        }
    }
    
    pub fn set_value(&mut self, value: Word) {
        self.value = value;
    }
    
    pub fn clear(&mut self) {
        self.value = 0;
    }
}