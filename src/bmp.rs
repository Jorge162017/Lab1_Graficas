use std::fs::File;
use std::io::Write;
use crate::framebuffer::FrameBuffer;
use crate::color::Color;

impl FrameBuffer {
    pub fn save_as_bmp(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        
        let file_header = create_file_header(self.width, self.height);
        let info_header = create_info_header(self.width, self.height);

        file.write_all(&file_header).unwrap();
        file.write_all(&info_header).unwrap();

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                file.write_all(&[pixel.b, pixel.g, pixel.r]).unwrap();
            }
        }
    }
}

fn create_file_header(width: usize, height: usize) -> [u8; 14] {
    let file_size = 14 + 40 + (width * height * 3) as u32;
    [
        0x42, 0x4D, // Signature
        (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8, ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8, // File size
        0x00, 0x00, 0x00, 0x00, // Reserved
        0x36, 0x00, 0x00, 0x00, // Offset to pixel data
    ]
}

fn create_info_header(width: usize, height: usize) -> [u8; 40] {
    [
        0x28, 0x00, 0x00, 0x00, // Header size
        (width & 0xFF) as u8, ((width >> 8) & 0xFF) as u8, ((width >> 16) & 0xFF) as u8, ((width >> 24) & 0xFF) as u8, // Width
        (height & 0xFF) as u8, ((height >> 8) & 0xFF) as u8, ((height >> 16) & 0xFF) as u8, ((height >> 24) & 0xFF) as u8, // Height
        0x01, 0x00, // Planes
        0x18, 0x00, // Bits per pixel (24)
        0x00, 0x00, 0x00, 0x00, // Compression (none)
        0x00, 0x00, 0x00, 0x00, // Image size (can be 0 for no compression)
        0x13, 0x0B, 0x00, 0x00, // X pixels per meter (2835)
        0x13, 0x0B, 0x00, 0x00, // Y pixels per meter (2835)
        0x00, 0x00, 0x00, 0x00, // Total colors (0 = 2^n)
        0x00, 0x00, 0x00, 0x00, // Important colors (0 = all)
    ]
}
