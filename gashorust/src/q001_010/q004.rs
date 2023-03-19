use image::{RgbImage, GrayImage, Luma};

pub fn run() {
    // Select an image file
    let img_path = "resources/imori.jpg";

    // Get GrayImage
    // You can get GrayImage from DynamicImage directory like this:
    // let gray = image::open(img_path).unwrap().to_luma8();
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();
    let gray: GrayImage = rgb2gray(&img);

    // to vec
    let mut vec = gray.to_vec();
    vec.sort();
    
    // initialize threshold
    let mut th: u8 = 0;
    let mut max_sb: f32 = 0.0;

    // find threshold
    for t in 0..255 {
        let mut w0: usize = 0;
        for pix in vec.iter() {
            if pix < &t {
                w0 += 1;
            } else {
                break
            }
        }
        let w1 = vec.len() as usize - w0;
        let m0 = (&vec[..w0]).iter().map(|&x| x as u32).sum::<u32>() as f32 / (&vec[..w0]).len() as f32;
        let m1 = (&vec[w0..]).iter().map(|&x| x as u32).sum::<u32>() as f32 / (&vec[w0..]).len() as f32;
        let sb = (w0 as f32 * w1 as f32 * (m0 - m1)).powi(2);

        if sb > max_sb {
            th = t;
            max_sb = sb;
        }
    }

    // Create new GrayImage
    let mut out_img: GrayImage = GrayImage::new(gray.width(), gray.height());

    for (x, y, pixel) in out_img.enumerate_pixels_mut() {
        let pix = gray.get_pixel(x, y);
        if pix[0] < th {
            *pixel = Luma([0]);
        } else {
            *pixel = Luma([255]);
        }
    }

    // Save image
    out_img.save("processed/result004.png").unwrap();

}

fn rgb2gray(img: &RgbImage) -> GrayImage {
    let mut gray: GrayImage = GrayImage::new(img.width(), img.height());
    for (x, y, pixel) in gray.enumerate_pixels_mut() {
        let pix = img.get_pixel(x, y);
        let lum: u8 = (pix[0] as f32 * 0.2126 +
                       pix[1] as f32 * 0.7152 +
                       pix[2] as f32 * 0.0722) as u8;
        *pixel = Luma([lum]);
    }
    gray
}