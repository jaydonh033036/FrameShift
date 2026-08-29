//! Add a docstring here in future

use image::RgbImage;

pub(crate) fn calculate_average(img: RgbImage) -> [u8;3] {
    let (r_counts, g_counts, b_counts) = border_arr(img);

    let r_total: u64 = r_counts
        .iter()
        .map(|&c| c as u64)
        .sum();

    let r_weighted_sum: u64 = r_counts
        .iter()
        .enumerate()
        .map(|(v, &c)| v as u64 * c as u64)
        .sum();
    let r_mean = (r_weighted_sum / r_total) as u8;

    let g_total: u64 = g_counts
        .iter()
        .map(|&c| c as u64)
        .sum();

    let g_weighted_sum: u64 = g_counts
        .iter()
        .enumerate()
        .map(|(v, &c)| v as u64 * c as u64)
        .sum();
    let g_mean = (g_weighted_sum / g_total) as u8;

    let b_total: u64 = b_counts
        .iter()
        .map(|&c| c as u64)
        .sum();

    let b_weighted_sum: u64 = b_counts
        .iter()
        .enumerate()
        .map(|(v, &c)| v as u64 * c as u64)
        .sum();
    let b_mean = (b_weighted_sum / b_total) as u8;

    [r_mean, g_mean, b_mean]
}

fn border_arr(img: RgbImage) -> ([u32;256],[u32;256],[u32;256]) {
    let mut r_counts = [0u32; 256];
    let mut g_counts = [0u32; 256];
    let mut b_counts = [0u32; 256];

    for (x, y, pixel) in img.enumerate_pixels() {
        let r = pixel.0[0];
        let g = pixel.0[1];
        let b = pixel.0[2];

        r_counts[r as usize] += 1;
        g_counts[g as usize] += 1;
        b_counts[b as usize] += 1;
    }

    return (r_counts, g_counts, b_counts)
}
