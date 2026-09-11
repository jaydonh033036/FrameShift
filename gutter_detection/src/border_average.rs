//! Add a docstring here in future

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use crate::Verbosity;


pub(crate) fn calculate_average(
    img: RgbImage,
    border_width: u8,
    verbosity: Verbosity,
    label: &str,
) -> [u8;3] {
    let (
        r_counts,
        g_counts,
        b_counts
    ) = border_arr(img, border_width as u32, verbosity, label);

    [
        median_from_counts(&r_counts),
        median_from_counts(&g_counts),
        median_from_counts(&b_counts),
    ]
}


fn border_arr(
    img: RgbImage,
    w: u32,
    verbosity: Verbosity,
    label: &str
) -> ([u32;256],[u32;256],[u32;256]) {
    let mut r_counts = [0u32; 256];
    let mut g_counts = [0u32; 256];
    let mut b_counts = [0u32; 256];

    let (width, height) = img.dimensions();

    // -v: one progress bar per image, ticked once per pixel visited (border or not).
    let progress = if verbosity == Verbosity::Verbose {
        let pb = ProgressBar::new((width as u64) * (height as u64));
        pb.set_style(
            ProgressStyle::with_template("{msg} [{bar:40}] {pos}/{len}")
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.set_message(label.to_string());
        Some(pb)
    } else {
        None
    };

    for (x, y, pixel) in img.enumerate_pixels() {
        if let Some(pb) = &progress {
            pb.inc(1);
        }

        let is_border = x < w || x >= (width - w) || y < w || y >= (height - w);

        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];

        if is_border {
            r_counts[r as usize] += 1;
            g_counts[g as usize] += 1;
            b_counts[b as usize] += 1;
        }
    }

    if let Some(pb) = progress {
        pb.finish_and_clear();
    }

    return (r_counts, g_counts, b_counts)
}


fn median_from_counts(counts: &[u32]) -> u8 {
    let total: u32 = counts.iter().sum();
    let mid = total / 2;
    let mut cumulative = 0u32;
    for (value, &count) in counts.iter().enumerate() {
        cumulative += count;
        if cumulative > mid {
            return value as u8;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    // Builds a `width` x `height` image where every pixel outside the given
    // border width is `interior_colour`, and every pixel within it is
    // `border_colour`. Mirrors the exact condition in `border_arr` so tests
    // stay meaningful if that condition changes.
    fn bordered_image(
        width: u32,
        height: u32,
        border_width: u32,
        border_colour: Rgb<u8>,
        interior_colour: Rgb<u8>,
    ) -> RgbImage {
        RgbImage::from_fn(width, height, |x, y| {
            let is_border = x < border_width
                || x >= width.saturating_sub(border_width)
                || y < border_width
                || y >= height.saturating_sub(border_width);
            if is_border { border_colour } else { interior_colour }
        })
    }


    mod median_from_counts_tests {
        use super::*;

        #[test]
        // Tests that if all the values are the same that it returns that value
        fn all_same_value() {
            let mut counts = [0u32; 256];
            counts[56] = 10;
            assert_eq!(median_from_counts(&counts), 56);
        }

        #[test]
        // Tests that an odd number of values returns the true median
        fn odd_total_picks() {
            let mut counts = [0u32; 256];
            counts[10] = 1;
            counts[20] = 1;
            counts[30] = 1;
            assert_eq!(median_from_counts(&counts), 20);
        }

        #[test]
        // Tests that an even number of values chooses the upper bound
        fn even_total_picks() {
            let mut counts = [0u32; 256];
            counts[50] = 5;
            counts[60] = 5;
            assert_eq!(median_from_counts(&counts), 60);
        }

        #[test]
        // values that span the entire range
        fn span_full_range() {
            let mut counts = [0u32; 256];
            counts[0] = 1;
            counts[255] = 2;
            assert_eq!(median_from_counts(&counts), 255);
        }

        #[test]
        #[should_panic]
        // Checks that the empty distribution panics
        fn empty_distribution() {
            let counts = [0u32; 256];
            median_from_counts(&counts);
        }
    }

    mod border_arr_tests {
        use super::*;

        #[test]
        // ensures only border pixels are counted and not interior pixels
        fn counts_only_border_pixels() {
            let border = Rgb([10, 20, 30]);
            let interior = Rgb([200, 200, 200]);
            let img = bordered_image(4, 4, 1, border, interior);

            let (r, g, b) = border_arr(
                img,
                1,
                Verbosity::Quiet,
                "test");

            // 4x4 image, border_width 1 -> interior is the 2x2 centre (4 pixels),
            // so 12 of the 16 pixels are border.
            assert_eq!(r[10], 12);
            assert_eq!(g[20], 12);
            assert_eq!(b[30], 12);
            // interior colour must not be counted at all
            assert_eq!(r[200], 0);
        }

        #[test]
        // ensures correct number of pixels are counted as border
        fn boundary_pixels_are_classified_correctly() {
            // 5x5, border_width 2 -> interior is exactly the single centre pixel (2,2).
            let border = Rgb([1, 1, 1]);
            let interior = Rgb([99, 99, 99]);
            let img = bordered_image(5, 5, 2, border, interior);

            let (r, _, _) = border_arr(img, 2, Verbosity::Quiet, "test");

            assert_eq!(r[1], 24); // everything except the one centre pixel
            assert_eq!(r[99], 0);
        }

        #[test]
        // checks correct number of pixels for non-square images
        fn non_square_image_does_not_swap_axes() {
            // Wide, short image: 6x3, border_width 1.
            // Interior would need width > 2 and height > 2; height=3 gives exactly
            // one interior row (y=1), width=6 gives interior x in 1-4 (4 pixels).
            let border = Rgb([5, 5, 5]);
            let interior = Rgb([250, 250, 250]);
            let img = bordered_image(6, 3, 1, border, interior);

            let (r, _, _) = border_arr(img, 1, Verbosity::Quiet, "test");

            // total = 18, interior = 4 -> border = 14
            assert_eq!(r[5], 14);
            assert_eq!(r[250], 0);
        }

        #[test]
        // tests that changing the verbosity has no effect on the outcome
        fn verbosity_does_not_affect_the_counts() {
            let border = Rgb([7, 8, 9]);
            let interior = Rgb([100, 100, 100]);

            let quiet = bordered_image(
                4,
                4,
                1,
                border,
                interior);
            let verbose = bordered_image(
                4,
                4,
                1,
                border,
                interior);

            let quiet_result = border_arr(quiet, 1, Verbosity::Quiet, "test");
            let verbose_result = border_arr(verbose, 1, Verbosity::Verbose, "test");

            assert_eq!(quiet_result, verbose_result);
        }

        #[test]
        // checks that if the border is wider than the image all pixels are counted as border
        fn border_wider_than_image_counts_everything() {
            let colour = Rgb([1, 2, 3]);
            let img = RgbImage::from_pixel(4, 4, colour);

            let (r, _, _) = border_arr(img, 100, Verbosity::Quiet, "test");

            assert_eq!(r[1], 16); // every pixel counted
        }
    }

    mod calculate_average_tests {
        use super::*;

        #[test]
        // ensures the three channels are averaged independently, not mixed
        fn combines_per_channel_medians_into_rgb_array() {
            let border = Rgb([10, 20, 30]);
            let interior = Rgb([200, 200, 200]);
            let img = bordered_image(4, 4, 1, border, interior);

            let result = calculate_average(
                img,
                1,
                Verbosity::Quiet,
                "test");

            assert_eq!(result, [10, 20, 30]);
        }
    }
}