use image::{Rgb, RgbImage};

pub fn run() {
    // Open image
    let img_path = "resources/imori.jpg";
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();

    // Create new image
    let mut out_img: RgbImage = RgbImage::new(img.width(), img.height());

    // Set pixel color
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pix = img.get_pixel(x, y);
            
            let r = (pix[0] / 64) * 64 + 32;
            let g = (pix[1] / 64) * 64 + 32;
            let b = (pix[2] / 64) * 64 + 32;
            let pixel = Rgb([r, g, b]);
            out_img.put_pixel(x, y, pixel);
        }
    }
    // Save image
    out_img.save("processed/result006.png").unwrap();
}