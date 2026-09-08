pub mod border_average;

use image::{ImageReader, ImageBuffer, Rgb};
use crate::border_average::{calculate_average, AverageType};

pub fn average(file_name: &str, avg_type: AverageType, border_width: u8) {
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let (width, height) = img.dimensions();
    let average_color = calculate_average(img, avg_type, border_width);
    let img_check = ImageBuffer::from_pixel(
        width,
        height,
        Rgb::from(average_color)
    );

    img_check.save("img_check.png").unwrap();


    println!("Average Colour: {:?}", average_color);
}