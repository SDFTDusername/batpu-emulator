use crate::machine::Word;

pub struct NumberDisplay {
    pub signed: bool,

    value: Word,
    value_updated: bool
}

impl NumberDisplay {
    pub fn new() -> Self {
        Self {
            signed: false,

            value: 0,
            value_updated: true
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
        self.value_updated = true;
    }
    
    pub fn clear(&mut self) {
        self.signed = false;

        self.value = 0;
        self.value_updated = true;
    }

    pub fn value_updated(&self) -> bool {
        self.value_updated
    }

    pub fn disable_value_updated(&mut self) {
        self.value_updated = false;
    }
}