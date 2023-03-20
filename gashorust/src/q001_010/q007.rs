use image::{Rgb, RgbImage};

const R: u32 = 8;

pub fn run() {
    // Open image
    let img_path = "resources/imori.jpg";
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();

    // Create new image
    let mut out_img: RgbImage = RgbImage::new(img.width(), img.height());

    let channels: usize = 3;
    // for large grid
    for y in 0..img.height() / R {
        for x in 0..img.width() / R {

            // for small grid (pooling)
            let mut v:[f32; 3] = [0.0; 3];
            for dy in 0..R {
                for dx in 0..R {
                    let pix = img.get_pixel(x * R + dx, y * R + dy);
                    for c in 0..channels {
                        v[c] += pix[c] as f32;
                    }
                }
            }
            let red = (v[0] / (R * R) as f32) as u8;
            let green = (v[1] / (R * R) as f32) as u8; 
            let blue = (v[2] / (R * R) as f32) as u8;
            let pixel = Rgb([red, green, blue]);

            // Set color
            for dy in 0..R {
                for dx in 0..R {
                    out_img.put_pixel(x * 8 + dx, y * 8 + dy, pixel);
                }
            }
        }
    }

    // Save image
    out_img.save("processed/result007.png").unwrap();
}