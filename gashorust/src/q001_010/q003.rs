use image::{GrayImage, Luma};

pub fn run() {
    // Open image
    let img_path = "resources/imori.jpg";
    let img = image::open(img_path).unwrap().to_rgb8();

    // Create new image
    let mut out_img = GrayImage::new(img.width(), img.height());

    // Set pixel luminance
    for (x, y, pixel) in out_img.enumerate_pixels_mut() {
        let pix = img.get_pixel(x, y);
        let mut lum: u8 = (pix[0] as f32 * 0.2126 +
                           pix[1] as f32 * 0.7152 +
                           pix[2] as f32 * 0.0722) as u8;
        lum = if lum < 128 { 0 } else { 255 };
        *pixel = Luma([lum]);
    }

    // Save image
    out_img.save("processed/result003.png").unwrap();
}