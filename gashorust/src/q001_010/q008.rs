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
            let mut v = [[0; (R*R) as usize]; 3];
            for dy in 0..R {
                for dx in 0..R {
                    let pix = img.get_pixel(x * R + dx, y * R + dy);
                    for c in 0..channels {
                        v[c][(dy*R+dx) as usize] = pix[c];
                    }
                }
            }

            // Sort by descending-order
            v[0].sort();
            v[0].reverse();
            v[1].sort();
            v[1].reverse();
            v[2].sort();
            v[2].reverse();
            let pixel = Rgb([v[0][0], v[1][0], v[2][0]]);

            // Set color
            for dy in 0..R {
                for dx in 0..R {
                    out_img.put_pixel(x * R + dx, y * R + dy, pixel);
                }
            }
        }
    }

    // Save image
    out_img.save("processed/result008.png").unwrap();
}