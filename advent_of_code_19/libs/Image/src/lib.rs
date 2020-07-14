#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::fs;
use std::fmt;

pub struct EncodedImage {
    size: (u32, u32)
    , page_size: u32
    , pixels: Vec<u32>
    , framebuffer: Vec<u32>
}

impl EncodedImage {
    pub fn new(filename: &str, size: (u32, u32)) -> EncodedImage {
        let mut framebuffer = Vec::with_capacity((size.0 * size.1) as usize);
        framebuffer.resize((size.0 * size.1) as usize, 2);
        EncodedImage {
            size
            , page_size: size.0 * size.1
            , pixels: EncodedImage::read_image(filename)
            , framebuffer
        }
    }

    fn read_image(filename: &str) -> Vec<u32>
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();

        contents.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    pub fn getPagesCount(&self) -> usize {
        self.pixels.len() / (self.size.0 * self.size.1) as usize
    }

    pub fn getPageSum(&self, pageId: usize) -> usize {
        let mut r = 0;
        for idx in (pageId * self.page_size as usize)..((pageId + 1) * self.page_size as usize) {
            r += self.pixels[idx] as usize;
        }
        r
    }

    pub fn getDigitsOnPageCount(&self, pageId: usize, digit: u32) -> usize {
        let mut r = 0;
        for idx in (pageId * self.page_size as usize)..((pageId + 1) * self.page_size as usize) {
            let val = self.pixels[idx];
            if val == digit {
                r += 1;
            }
        }
        r
    }

    pub fn printPage(&self, pageId: usize) {
        println!("Page: {}", pageId);
        for idx in (pageId * self.page_size as usize)..((pageId + 1) * self.page_size as usize) {
            print!("{}", self.pixels[idx]);
            if idx % self.size.0 as usize == 0 {
                println!();
            }
        }
        println!();
    }

    pub fn printBlended(&self) {
        print!("Blended image:");
        for idx in 0..self.page_size {
            if idx % self.size.0 == 0 {
                println!();
            }
            let m = match self.framebuffer[idx as usize] {
                0 => " ",
                _ => "*"
            };
            print!("{}", m);
        }
        println!();
    }

    pub fn blendToPage(&mut self, pageId: usize) {
        for pixel_idx in 0..self.page_size {
            let mut pixel_value = 2; // - 1 layer is transparent
            for page_idx in 0..pageId {
                let idx = page_idx * self.page_size as usize + pixel_idx as usize;
                pixel_value = match pixel_value {
                    0 => pixel_value,
                    1 => pixel_value,
                    _ => self.pixels[idx]
                };
                println!("Page {} Raw {} Column {} Value {}", page_idx, pixel_idx / self.size.0, pixel_idx % self.size.0, pixel_value);
            }
            self.framebuffer[pixel_idx as usize] = pixel_value;
        }
    }

    pub fn blend(&mut self) {
        self.blendToPage( self.getPagesCount())
    }
}

impl fmt::Display for EncodedImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.size, self.pixels)
    }
}