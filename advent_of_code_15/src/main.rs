use Image;
use std::cmp::max;

fn main() {
    let image = Image::EncodedImage::new("input.txt", (25, 6));
    let mut r = (0, 1000000);
    for idx in 0..image.getPagesCount() {
        let zeros = image.getDigitsOnPageCount(idx as usize, 0);
        if r.1 > zeros {
            r.0 = idx;
            r.1 = zeros;
        }
    }

    let result = image.getDigitsOnPageCount(r.0 as usize, 1)
        * image.getDigitsOnPageCount(r.0 as usize, 2);
    println!("Result {:?}", result);
}
