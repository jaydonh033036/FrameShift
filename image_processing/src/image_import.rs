use image::{ImageReader,RgbImage, Rgb};

pub(crate) fn import(file_name: &str) {
    let mut img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let width = img.width();
    let height = img.height();

    println!("Width: {}", width);
    println!("Height: {} \n", height);

    let mut num_pixels:u64 = 0;
    let mut tot_rgb:[u64; 3] = [0, 0, 0];

    for pixel in img.pixels() {
        let [r, g, b] = pixel.0;

        num_pixels += 1;

        tot_rgb[0] += r as u64;
        tot_rgb[1] += g as u64;
        tot_rgb[2] += b as u64;
    }

    let avg_colour:[u8;3] = [
        (tot_rgb[0] / num_pixels) as u8,
        (tot_rgb[1] / num_pixels) as u8,
        (tot_rgb[2] / num_pixels) as u8,
    ];

    println!("Total Pixels: {}", num_pixels);
    println!("Average Colour: {:?}", avg_colour);

    let mut avg_image = RgbImage::new(width, height);

    for pixel in avg_image.pixels_mut() {
        *pixel = Rgb(avg_colour);
    }

    avg_image.save("average.png").unwrap();
}