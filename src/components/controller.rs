use crate::machine::Word;

pub struct Controller {
    pub start: bool,
    pub select: bool,

    pub a: bool,
    pub b: bool,

    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool
}

impl Controller {
    pub fn new() -> Self {
        Self {
            start: false,
            select: false,
            
            a: false,
            b: false,
            
            up: false,
            right: false,
            down: false,
            left: false
        }
    }
    
    pub fn clear(&mut self) {
        self.start = false;
        self.select = false;

        self.a = false;
        self.b = false;

        self.up = false;
        self.right = false;
        self.down = false;
        self.left = false;
    }
    
    pub fn binary(&self) -> Word {
        let mut binary = 0;
        
        binary |= (self.start as Word) << 7;
        binary |= (self.select as Word) << 6;

        binary |= (self.a as Word) << 5;
        binary |= (self.b as Word) << 4;
        
        binary |= (self.up as Word) << 3;
        binary |= (self.right as Word) << 2;
        binary |= (self.down as Word) << 1;
        binary |= self.left as Word;
        
        binary
    }
}