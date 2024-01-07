extern crate image;

use image::{DynamicImage, GenericImageView, Rgba};

fn color_distance(&pixel: &Rgba<u8>, &reference_pixel: &Rgba<u8>) -> f64 {
    let [r1, g1, b1, a1] = pixel.0;
    let [r2, g2, b2, a2] = reference_pixel.0;
    let r = (r1 as i16 - r2 as i16) as f64;
    let g = (g1 as i16 - g2 as i16) as f64;
    let b = (b1 as i16 - b2 as i16) as f64;
    let a = (a1 as i16 - a2 as i16) as f64;
    let distance = (r * r + g * g + b * b + a * a).sqrt();
    distance
}

fn process_line<F>(length: u32, threshold: f64, &reference_pixel: &Rgba<u8>, mut get_pixel: F) -> u32
where F: FnMut(u32) -> Rgba<u8>,
{
    let mut counter = 0;
    for pos in 0..length {
        let pixel = get_pixel(pos);
        let distance = color_distance(&pixel, &reference_pixel);
        if distance > threshold {
            counter += 1;
        }
    }
    counter
}

pub fn find_margins(img: &DynamicImage, threshold: f64) -> ((u32, u32), (u32, u32)) {
    let (width, height) = img.dimensions();
    let top_left_pixel = img.get_pixel(0, 0);

    // Refactored margin finding logic
    let top_margin = (0..height)
        .find(|&y| {
            process_line(width, threshold, &top_left_pixel, |x| img.get_pixel(x, y)) as f64 / width as f64 > 0.1
        })
        .unwrap_or(height)
        - 1;

    let left_margin = (0..width)
        .find(|&x| {
            process_line(height, threshold, &top_left_pixel, |y| img.get_pixel(x, y)) as f64 / height as f64 > 0.1
        })
        .unwrap_or(width)
        - 1;

    let bottom_margin = (0..height)
        .rev()
        .find(|&y| {
            process_line(width, threshold, &top_left_pixel, |x| img.get_pixel(x, y)) as f64 / width as f64 > 0.1
        })
        .unwrap_or(0)
        + 1;

    let right_margin = (0..width)
        .rev()
        .find(|&x| {
            process_line(height, threshold, &top_left_pixel, |y| img.get_pixel(x, y)) as f64 / height as f64 > 0.1
        })
        .unwrap_or(0)
        + 1;

    ((left_margin, top_margin), (right_margin, bottom_margin))
}

// generate unit test for find_empty_space
#[cfg(test)]
mod tests {
    use image::GenericImage;

    use crate::image_actions::find_empty_space::find_margins;

    #[test]
    fn test_find_top_margin() {
        // a green square in the middle of the image
        let mut img = image::DynamicImage::new_rgb8(300, 300);
        for x in 100..200 {
            for y in 100..200 {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }

        let margins = find_margins(&mut img, 100.0);

        assert_eq!(margins.0 .1, 99);
    }

    #[test]
    fn test_find_left_margin() {
        // a green square in the middle of the image
        let mut img = image::DynamicImage::new_rgb8(300, 300);
        for x in 100..200 {
            for y in 100..200 {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }

        let margins = find_margins(&mut img, 100.0);

        assert_eq!(margins.0 .0, 99);
    }

    #[test]
    fn test_find_bottom_right_margin() {
        // a green square in the middle of the image
        let mut img = image::DynamicImage::new_rgb8(300, 300);
        for x in 100..200 {
            for y in 100..200 {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }

        let margins = find_margins(&mut img, 100.0);

        assert_eq!(margins.1 .0, 200);
        assert_eq!(margins.1 .1, 200);
    }
}
