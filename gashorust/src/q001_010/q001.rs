use image::{Rgb, RgbImage, GenericImageView};

pub fn run() {
    // Open image
    let img_path = "resources/imori.jpg";
    let img = image::open(img_path).unwrap();

    // Create new image
    let mut out_img = RgbImage::new(img.width(), img.height());

    // Set pixel color
    for (x, y, pixel) in out_img.enumerate_pixels_mut() {
        let pix = img.get_pixel(x, y);
        *pixel = Rgb([pix[2], pix[1], pix[0]]);
    }

    // Another answer
    // for y in 0..img.height() {
    //     for x in 0..img.width() {
    //         let pix = img.get_pixel(x, y);
    //         let pixel: Rgb<u8> = Rgb([pix[2], pix[1], pix[0]]);
    //         out_img.put_pixel(x, y, pixel);
    //     }
    // }

    // Save image
    out_img.save("processed/result001.png").unwrap();
}