//! Add a docstring here in future

use image::RgbImage;
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};
use crate::Verbosity;

/// Statistical method used to summarize border pixel colours.
#[derive(Clone, Copy, ValueEnum)]
pub enum AverageType {
    /// Arithmetic mean of all border pixel values per channel.
    Mean,
    /// Middle value of all border pixel values per channel, once sorted.
    Median,
    /// Most frequently occurring value among border pixels per channel.
    Mode,
}


impl std::fmt::Display for AverageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AverageType::Mean => write!(f, "mean"),
            AverageType::Median => write!(f, "median"),
            AverageType::Mode => write!(f, "mode"),
        }
    }
}


pub(crate) fn calculate_average(
    img: RgbImage,
    avg_type: AverageType,
    border_width: u8,
    verbosity: Verbosity,
    label: &str,
) -> [u8;3] {
    let (
        r_counts,
        g_counts,
        b_counts
    ) = border_arr(img, border_width as u32, verbosity, label);

    match avg_type {
        AverageType::Mean => [
            mean_from_counts(&r_counts),
            mean_from_counts(&g_counts),
            mean_from_counts(&b_counts),
        ],
        AverageType::Median => [
            median_from_counts(&r_counts),
            median_from_counts(&g_counts),
            median_from_counts(&b_counts),
        ],
        AverageType::Mode => [
            mode_from_counts(&r_counts),
            mode_from_counts(&g_counts),
            mode_from_counts(&b_counts),
        ],
    }
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


fn mean_from_counts(counts: &[u32;256]) -> u8 {
    let total: u64 = counts
        .iter()
        .map(|&c| c as u64)
        .sum();

    let weighted_sum: u64 = counts
        .iter()
        .enumerate()
        .map(|(v, &c)| v as u64 * c as u64)
        .sum();

    (weighted_sum / total) as u8
}


fn mode_from_counts(counts: &[u32; 256]) -> u8 {
    counts
        .iter()
        .enumerate()
        .max_by_key(|&(_, &c)| c)
        .map(|(v, _)| v as u8)
        .unwrap()
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