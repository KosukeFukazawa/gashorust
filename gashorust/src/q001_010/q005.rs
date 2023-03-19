use image::{RgbImage, Rgb};

pub fn run() {
    // Select an image file
    let img_path = "resources/imori.jpg";
    let img: RgbImage = image::open(img_path).unwrap().to_rgb8();

    let mut img_max: Vec<Vec<f32>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];
    let mut img_min: Vec<Vec<f32>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];
    let mut arg_min: Vec<Vec<usize>> = vec![vec![0; img.width() as usize]; img.height() as usize];
    
    // Extract min / max
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pix = img.get_pixel(x, y);
            let max_value = *[pix[0], pix[1], pix[2]].iter().max().unwrap() as f32;
            let min_value = *[pix[0], pix[1], pix[2]].iter().min().unwrap() as f32;
            let min_index = [pix[0], pix[1], pix[2]].iter().enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().0;
            img_max[x as usize][y as usize] = if max_value == 0.0 { 0.0 } else { max_value / 255.0 };
            img_min[x as usize][y as usize] = if min_value == 0.0 { 0.0 } else { min_value / 255.0 };
            arg_min[x as usize][y as usize] = min_index;
        }
    }

    // Create HSV
    let mut h: Vec<Vec<f32>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];
    let mut s: Vec<Vec<f32>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];
    let mut v: Vec<Vec<f32>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];

    // RGB -> HSV
    for y in 0..img.height() as usize {
        for x in 0..img.width() as usize {
            // max - min
            let mmm = img_max[x][y] - img_min[x][y];
            let pix = img.get_pixel(x as u32, y as u32);

            // Set H
            if mmm == 0.0 {
                h[x][y] = 0.0;
            } else if arg_min[x][y] == 0 {
                h[x][y] = 60.0 * (pix[2] as f32 - pix[1] as f32) / (mmm * 255.0) + 180.0;
            } else if arg_min[x][y] == 1 {
                h[x][y] = 60.0 * (pix[0] as f32 - pix[2] as f32) / (mmm * 255.0) + 300.0;
            } else if arg_min[x][y] == 2 {
                h[x][y] = 60.0 * (pix[1] as f32 - pix[0] as f32) / (mmm * 255.0) + 60.0;
            }
            // Set S
            s[x][y] = mmm;
            // Set V
            v[x][y] = img_max[x][y];
        }
    }

    let mut out_img: RgbImage = RgbImage::new(img.width(), img.height());

    // HSV -> RGB
    for y in 0..img.height() as usize {
        for x in 0..img.width() as usize {
            let c = s[x][y];
            // hue reversal
            let hprime = ((h[x][y] + 180.0) % 360.0) / 60.0;
            let xprime = c * (1.0 - (hprime % 2.0 - 1.0).abs());

            //rgb
            let mut r:u8 = 0;
            let mut g:u8 = 0;
            let mut b:u8 = 0;
            if hprime < 1.0 {
                r = (c * 255.0) as u8;
                g = (xprime * 255.0) as u8;
            } else if hprime < 2.0 {
                r = (xprime * 255.0) as u8;
                g = (c * 255.0) as u8;
            } else if hprime < 3.0 {
                g = (c * 255.0) as u8;
                b = (xprime * 255.0) as u8;
            } else if hprime < 4.0 {
                g = (xprime * 255.0) as u8;
                b = (c * 255.0) as u8;
            } else if hprime < 5.0 {
                r = (xprime * 255.0) as u8;
                b = (c * 255.0) as u8;
            } else if hprime < 6.0 {
                r = (c * 255.0) as u8;
                b = (xprime * 255.0) as u8;
            }
            let pixel = Rgb([r, g, b]);

            out_img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // Save image
    out_img.save("processed/result005.png").unwrap();
}