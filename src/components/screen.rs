use egui::{Color32, ColorImage};

pub struct Screen {
    pub x: isize,
    pub y: isize,
    
    width: usize,
    height: usize,
    
    buffer: Vec<u8>,
    
    image: ColorImage,
    image_updated: bool,

    pub off_color: Color32,
    pub on_color: Color32
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let data_length = ((width * height) as f32 / 8.0).ceil() as usize;

        let off_color = Color32::from_rgb(158, 92, 56);
        let on_color = Color32::from_rgb(242, 176, 111);

        let image = ColorImage::new([width, height], off_color);

        Self {
            x: 0,
            y: 0,
            
            width,
            height,
            
            buffer: vec![0; data_length],
            
            image,
            image_updated: true,

            off_color,
            on_color
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
        for y in 0..self.height {
            for x in 0..self.width {
                let (byte, bit) = self.get_index(x as isize, y as isize);

                let color = if ((self.buffer[byte] >> bit) & 1) != 0 {
                    self.on_color
                } else {
                    self.off_color
                };

                self.image[(x, (self.height - 1 - y))] = color;
            }
        }

        self.image_updated = true;
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0);

        let image = ColorImage::new([self.width, self.height], self.off_color);
        self.image = image;
        self.image_updated = true;
    }

    pub fn image(&self) -> ColorImage {
        self.image.clone()
    }
    
    pub fn image_updated(&self) -> bool {
        self.image_updated
    }
    
    pub fn disable_image_updated(&mut self) {
        self.image_updated = false;
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