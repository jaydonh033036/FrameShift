pub mod border_average;

use image::{ImageReader};
use crate::border_average::calculate_average;

pub fn average(file_name: &str) {
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let average_color = calculate_average(img);

    println!("Average Colour: {:?}", average_color);
}