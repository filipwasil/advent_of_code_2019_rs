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
    , pixels: Vec<u32>
}

impl EncodedImage {
    pub fn new(filename: &str, size: (u32, u32)) -> EncodedImage {
        let pixels = EncodedImage::read_image(filename);
        EncodedImage {
            size
            , pixels
        }
    }

    fn read_image(filename: &str) -> Vec<u32>
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();

        contents.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    pub fn getPagesCount(&self) -> u32 {
        self.pixels.len() as u32 / (self.size.0 * self.size.1)
    }

    pub fn getPageSum(&self, pageId: usize) -> usize {
        let pageSize = self.size.0 * self.size.1;
        let mut r = 0;
        // todo functional style needed
        for idx in (pageId * pageSize as usize)..((pageId + 1) * pageSize as usize) {
            r += self.pixels[idx] as usize;
        }
        r
    }

    pub fn getDigitsOnPageCount(&self, pageId: usize, digit: u32) -> usize {
        let pageSize = self.size.0 * self.size.1;
        let mut r = 0;
        // todo functional style needed
        for idx in (pageId * pageSize as usize)..((pageId + 1) * pageSize as usize) {
            let val = self.pixels[idx];
            if val == digit {
                r += 1;
            }
        }
        r
    }

    pub fn printPage(&self, pageId: usize) {
        let pageSize = self.size.0 * self.size.1;
        println!("Page: {}", pageId);
        for idx in (pageId * pageSize as usize)..((pageId + 1) * pageSize as usize) {
            print!("{}", self.pixels[idx]);
            if idx % self.size.0 as usize == 0 {
                println!();
            }
        }
        println!();
    }
}

impl fmt::Display for EncodedImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.size, self.pixels)
    }
}