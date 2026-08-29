pub mod border_average;

use image::{ImageReader};
use crate::border_average::{calculate_average, AverageType};
use crate::border_average::AverageType::Mean;

pub fn average(file_name: &str, avg_type: AverageType) {
    let img = ImageReader::open(file_name)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let average_color = calculate_average(img, avg_type);

    println!("Average Colour: {:?}", average_color);
}