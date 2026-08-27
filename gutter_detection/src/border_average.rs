//! Add a docstring here in future

fn num_pixel_border(width: u32, height: u32, thickness: u32) -> u32 {
    if 2 * thickness >= width.min(height) {
        return width * height;
    }

    2 * thickness * (width + height - 2 * thickness)
}

#[cfg(test)]
mod num_pixel_border_tests {
    use super::*;

    #[test]
    fn single_pixel_ring() {
        // thickness = 1 on a 10x10 image: standard ring perimeter, 2*10+2*10-4
        assert_eq!(num_pixel_border(10, 10, 1), 36);
    }

    #[test]
    fn two_pixel_ring() {
        // outer ring (36) + next ring in, a 8x8 ring (28)
        assert_eq!(num_pixel_border(10, 10, 2), 64);
    }

    #[test]
    fn non_square_image() {
        assert_eq!(num_pixel_border(20, 10, 3), 144);
    }

    #[test]
    fn thickness_covers_whole_image_exactly() {
        // 2*thickness == min(width, height): whole image is border
        assert_eq!(num_pixel_border(10, 10, 5), 100);
    }

    #[test]
    fn thickness_exceeds_image_size() {
        assert_eq!(num_pixel_border(5, 5, 10), 25);
    }

    #[test]
    fn zero_thickness_is_zero_border() {
        assert_eq!(num_pixel_border(10, 10, 0), 0);
    }

    #[test]
    fn zero_sized_image() {
        assert_eq!(num_pixel_border(0, 10, 3), 0);
    }
}
