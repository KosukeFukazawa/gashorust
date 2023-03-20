use image::{Rgb, RgbImage};

const C: usize = 3;

pub fn run() {
    // Open noisy image
    let img_path = "resources/imori_noise.jpg";
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();

    let out_img: RgbImage = median_filter(&img, 3);

    // Save image
    out_img.save("processed/result010.png").unwrap();
}

fn median_filter(img: &RgbImage, kernel_size: u32) -> RgbImage {
    let mut out_img: RgbImage = RgbImage::new(img.width(), img.height());
    
    let pad = kernel_size / 2;

    // Filtering image
    for y in 0..img.height() { 
        for x in 0..img.width() {
            let mut v: Vec<Vec<u8>> = vec![Vec::new(); 3];

            for dy in 0..kernel_size {
                let _y = (y + dy) as i16 - pad as i16;
                if _y >= 0 && (_y as u32) < img.height() {
                    for dx in 0..kernel_size {
                        let _x = (x + dx) as i16 - pad as i16;
                        if _x >= 0 && (_x as u32) < img.width() {
                            let pix = img.get_pixel(_x as u32, _y as u32);
                            for c in 0..C {
                                v[c].push(pix[c]);
                            }
                        } else {
                            for c in 0..C {
                                v[c].push(0);
                            }
                        }
                    }
                } else {
                    for c in 0..C {
                        for _ in 0..kernel_size as usize {
                            v[c].push(0);
                        }
                    }
                }
            }
            
            let mut median: [u8; C] = [0; C];
            for c in 0..C {
                v[c].sort();
                let center: usize = (kernel_size * kernel_size) as usize / 2;
                median[c] = v[c][center];
            }

            let pixel = Rgb([median[0], median[1], median[2]]);
            out_img.put_pixel(x, y, pixel);
        }
    }

    out_img
}