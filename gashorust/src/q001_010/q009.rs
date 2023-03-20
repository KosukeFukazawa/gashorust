use std::f32::consts::PI;

use image::{Rgb, RgbImage};

const C: usize = 3;

pub fn run() {
    // Open noisy image
    let img_path = "resources/imori_noise.jpg";
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();

    let out_img: RgbImage = gaussian_filter(&img, 3, 1.2);

    // Save image
    out_img.save("processed/result009.png").unwrap();
}

fn gaussian_filter(img: &RgbImage, kernel_size: usize, sigma: f32) -> RgbImage {
    let mut out_img: RgbImage = RgbImage::new(img.width(), img.height());
    let mut kernel = vec![vec![0.0; kernel_size]; kernel_size];

    let mut sum: f32 = 0.0;
    let pad = (kernel_size / 2) as f32;

    // Make filter
    for y in 0..kernel_size {
        for x in 0..kernel_size {
            let dis_y = y as f32 - pad;
            let dis_x = x as f32 - pad;

            kernel[y][x] = 1.0 / (2.0 * PI * sigma * sigma) *
                (- (dis_x * dis_x + dis_y * dis_y) / (2.0 * sigma * sigma)).exp();
            sum += kernel[y][x];
        }
    }

    for y in 0..kernel_size {
        for x in 0..kernel_size {
            kernel[y][x] /= sum;
        }
    }

    // Filtering image
    for y in 0..img.height() { 
        for x in 0..img.width() {
            let mut v: [f32; 3] = [0.0; 3];
            for dy in 0..kernel_size {
                let _y = (y + dy as u32) as i16 - pad as i16;
                if _y >= 0 && (_y as u32) < img.height() {
                    for dx in 0..kernel_size {
                        let _x = (x + dx as u32) as i16 - pad as i16;
                        if _x >= 0 && (_x as u32) < img.width() {
                            let pix = img.get_pixel(_x as u32, _y as u32);
                            for c in 0..C {
                                v[c] += kernel[dy][dx] * pix[c] as f32;
                            }
                        }
                    }
                }

            }
            let pixel = Rgb([v[0] as u8, v[1] as u8, v[2] as u8]);
            out_img.put_pixel(x, y, pixel);
        }
    }

    out_img
}