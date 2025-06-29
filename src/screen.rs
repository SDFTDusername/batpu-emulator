pub struct Screen {
    pub x: isize,
    pub y: isize,
    
    width: usize,
    height: usize,
    
    buffer: Vec<u8>,
    data: Vec<u8>
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let data_length = ((width * height) as f32 / 8.0).ceil() as usize;
        
        Self {
            x: 0,
            y: 0,
            
            width,
            height,
            
            buffer: vec![0; data_length],
            data: vec![0; data_length]
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn pix(&self) -> bool {
        let (byte, bit) = self.get_index(self.x, self.y);
        
        self.data[byte] & (1 << bit) != 0
    }

    pub fn set_pix(&mut self, value: bool) {
        let (byte, bit) = self.get_index(self.x, self.y);
        
        if (value) {
            self.data[byte] |= 1 << bit;
        } else {
            self.data[byte] &= !(1 << bit);
        }
    }
    
    pub fn push_buffer(&mut self) {
        self.data.copy_from_slice(&self.buffer);
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.data.fill(0);
    }
    
    fn get_index(&self, x: isize, y: isize) -> (usize, usize) {
        let x = x.rem_euclid(self.width as isize);
        let y = y.rem_euclid(self.height as isize);

        let i = x + y * self.width as isize;

        let byte = (i / 8) as usize;
        let bit = (i % 8) as usize;

        (byte, bit)
    }
}