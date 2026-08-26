use image::{ImageReader,RgbImage, Rgb};

pub fn average(file_name: &str) {
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let width = img.width() as u64;
    let height = img.height() as u64;

    println!("Width: {}", width);
    println!("Height: {} \n", height);

    let  num_pixels:u64 = width * height;
    let mut tot_rgb:[u64; 3] = [0, 0, 0];

    for pixel in img.pixels() {
        let [r, g, b] = pixel.0;

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
}
