use iced::advanced::image::Bytes;
use iced::widget::image::Handle;
use iced::Color;

pub struct Screen {
    pub x: isize,
    pub y: isize,
    
    width: usize,
    height: usize,
    
    buffer: Vec<u8>,
    handle: Handle,

    pub off_color: Color,
    pub on_color: Color
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let data_length = ((width * height) as f32 / 8.0).ceil() as usize;

        let off_color = Color::from_rgb8(158, 92, 56);
        let on_color = Color::from_rgb8(242, 176, 111);

        let bytes = Self::gen_rgba(
            width, height,
            off_color, on_color,
            &vec![0; width * height]
        );

        let handle = Handle::from_rgba(width as u32, height as u32, bytes);

        Self {
            x: 0,
            y: 0,
            
            width,
            height,
            
            buffer: vec![0; data_length],
            handle,

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
        let (byte, bit) = Self::get_index(self.width as isize, self.height as isize, self.x, self.y);

        ((self.buffer[byte] >> bit) & 1) != 0
    }

    pub fn set_pix(&mut self, value: bool) {
        let (byte, bit) = Self::get_index(self.width as isize, self.height as isize, self.x, self.y);
        
        if value {
            self.buffer[byte] |= 1 << bit;
        } else {
            self.buffer[byte] &= !(1 << bit);
        }
    }
    
    pub fn push_buffer(&mut self) {
        let bytes = Self::gen_rgba(
            self.width, self.height,
            self.off_color, self.on_color,
            &self.buffer
        );

        let handle = Handle::from_rgba(self.width as u32, self.height as u32, bytes);
        self.handle = handle;
    }
    
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0);

        let handle = Handle::from_rgba(
            self.width as u32, self.height as u32,
            Self::gen_rgba(
                self.width, self.height,
                self.off_color, self.on_color,
                &vec![0; self.width * self.height]
            )
        );

        self.handle = handle;
    }

    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    fn gen_rgba(width: usize, height: usize, off_color: Color, on_color: Color, data: &[u8]) -> Bytes {
        let mut bytes: Vec<u8> = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            for x in 0..width {
                let (byte, bit) = Self::get_index(width as isize, height as isize, x as isize, (height - 1 - y) as isize);

                if ((data[byte] >> bit) & 1) != 0 {
                    bytes.push((on_color.r * 255.0) as u8);
                    bytes.push((on_color.g * 255.0) as u8);
                    bytes.push((on_color.b * 255.0) as u8);
                    bytes.push((on_color.a * 255.0) as u8);
                } else {
                    bytes.push((off_color.r * 255.0) as u8);
                    bytes.push((off_color.g * 255.0) as u8);
                    bytes.push((off_color.b * 255.0) as u8);
                    bytes.push((off_color.a * 255.0) as u8);
                }
            }
        }

        Bytes::copy_from_slice(&bytes)
    }
    
    fn get_index(width: isize, height: isize, x: isize, y: isize) -> (usize, usize) {
        let x = x.rem_euclid(width);
        let y = y.rem_euclid(height);

        let i = x + y * width;

        let byte = (i / 8) as usize;
        let bit = (i % 8) as usize;

        (byte, bit)
    }
}