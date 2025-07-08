pub struct Screen {
    pub x: isize,
    pub y: isize,
    
    width: usize,
    height: usize,
    
    buffer: Vec<u8>,
    
    image: Vec<u8>,
    image_updated: bool
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
            
            image: vec![0; data_length],
            image_updated: true
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

        ((self.buffer[byte] >> bit) & 1) != 0
    }

    pub fn set_pix(&mut self, value: bool) {
        let (byte, bit) = self.get_index(self.x, self.y);
        
        if value {
            self.buffer[byte] |= 1 << bit;
        } else {
            self.buffer[byte] &= !(1 << bit);
        }
    }
    
    pub fn push_buffer(&mut self) {
        self.image.copy_from_slice(&self.buffer);
        self.image_updated = true;
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0);
        
        self.image.fill(0);
        self.image_updated = true;
    }

    pub fn image(&self) -> &[u8] {
        &self.image
    }
    
    pub fn image_updated(&self) -> bool {
        self.image_updated
    }
    
    pub fn disable_image_updated(&mut self) {
        self.image_updated = false;
    }
    
    pub fn get_index(&self, x: isize, y: isize) -> (usize, usize) {
        let x = x.rem_euclid(self.width as isize);
        let y = y.rem_euclid(self.height as isize);

        let i = x + y * self.width as isize;

        let byte = (i / 8) as usize;
        let bit = (i % 8) as usize;

        (byte, bit)
    }
}