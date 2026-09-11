pub mod border_average;

use image::{ImageReader, ImageBuffer, Rgb};
use crate::border_average::{calculate_average};

/// Controls how much a single image's processing prints to stdout.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Nothing is printed at all.
    Quiet,
    /// Nothing extra is printed beyond errors.
    Normal,
    /// A progress bar is shown while scanning border pixels, then the average colour.
    Verbose,
}

pub fn average(
    input_file: &str,
    output_file: Option<&str>,
    border_width: u8,
    verbosity: Verbosity,
) {
    let img = ImageReader::open(input_file)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let (width, height) = img.dimensions();
    let average_color = calculate_average(
        img,
        border_width, 
        verbosity, 
        input_file
    );

    if let Some(out) = output_file {
        let img_check = ImageBuffer::from_pixel(
            width, 
            height, 
            Rgb::from(average_color)
        );
        img_check.save(out).unwrap();
    }

    if verbosity != Verbosity::Quiet {
        println!("{input_file}: Average Colour: {average_color:?}");
    }
}