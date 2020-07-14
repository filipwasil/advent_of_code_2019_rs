use Image;

fn main() {
    let mut image = Image::EncodedImage::new("input.txt", (25, 6));
    image.blend();
    image.printBlended();
}
