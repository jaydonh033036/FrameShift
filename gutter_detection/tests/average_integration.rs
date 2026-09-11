use gutter_detection::{average, Verbosity};
use image::{Rgb, RgbImage};
use tempfile::tempdir;

// Writes a small square fixture image with a solid border colour and a
// solid, distinct interior colour, at the given path.
fn write_bordered_fixture(path: &std::path::Path, border: Rgb<u8>, interior: Rgb<u8>) {
    let img = RgbImage::from_fn(6, 6, |x, y| {
        let is_border = x < 1 || x >= 5 || y < 1 || y >= 5;
        if is_border { border } else { interior }
    });
    img.save(path).unwrap();
}

#[test]
// the saved output image should be filled with the computed average colour
fn saves_output_file_with_expected_average_colour() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.png");
    let output_path = dir.path().join("output.png");

    write_bordered_fixture(&input_path, Rgb([10, 20, 30]), Rgb([200, 200, 200]));

    average(
        input_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
        1,
        Verbosity::Quiet,
    );

    assert!(output_path.exists());

    let saved = image::open(&output_path).unwrap().to_rgb8();
    // every pixel in the output should be the border colour, since that's
    // the only colour present at the border and thus the median.
    assert_eq!(*saved.get_pixel(0, 0), Rgb([10, 20, 30]));
    assert_eq!(*saved.get_pixel(3, 3), Rgb([10, 20, 30]));
}

#[test]
// no output_file argument should mean no file is written at all
fn does_not_create_output_file_when_none_given() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.png");
    let output_path = dir.path().join("should_not_exist.png");

    write_bordered_fixture(&input_path, Rgb([1, 2, 3]), Rgb([9, 9, 9]));

    average(input_path.to_str().unwrap(), None, 1, Verbosity::Quiet);

    assert!(!output_path.exists());
}

#[test]
// the saved output image should match the input's dimensions
fn output_image_matches_input_dimensions() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.png");
    let output_path = dir.path().join("output.png");

    write_bordered_fixture(&input_path, Rgb([50, 50, 50]), Rgb([150, 150, 150]));

    average(
        input_path.to_str().unwrap(),
        Some(output_path.to_str().unwrap()),
        1,
        Verbosity::Quiet,
    );

    let saved = image::open(&output_path).unwrap().to_rgb8();
    assert_eq!(saved.dimensions(), (6, 6));
}

#[test]
// verbosity should have no effect on the file that gets written
fn verbosity_does_not_affect_saved_output() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.png");
    let quiet_out = dir.path().join("quiet.png");
    let verbose_out = dir.path().join("verbose.png");

    write_bordered_fixture(&input_path, Rgb([7, 8, 9]), Rgb([100, 100, 100]));

    average(input_path.to_str().unwrap(), Some(quiet_out.to_str().unwrap()), 1, Verbosity::Quiet);
    average(input_path.to_str().unwrap(), Some(verbose_out.to_str().unwrap()), 1, Verbosity::Verbose);

    let quiet_pixels = image::open(&quiet_out).unwrap().to_rgb8();
    let verbose_pixels = image::open(&verbose_out).unwrap().to_rgb8();
    assert_eq!(quiet_pixels, verbose_pixels);
}

#[test]
#[should_panic]
// documents current behaviour: a missing input file panics rather than
// returning an error
fn panics_on_missing_input_file() {
    average("this/path/does/not/exist.png", None, 1, Verbosity::Quiet);
}