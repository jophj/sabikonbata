extern crate image;

use image::{imageops, DynamicImage};

pub fn crop(
    img: &mut image::DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> image::SubImage<&mut DynamicImage> {
    imageops::crop(img, x, y, width, height)
}

#[cfg(test)]
mod tests {
    use image::GenericImageView;

    use crate::image_actions::crop;

    #[test]
    fn test_crop_image() {
        let mut img = image::DynamicImage::new_rgb8(400, 420);
        assert_eq!(img.width(), 400);
        assert_eq!(img.height(), 420);

        let (x, y, width, height) = (0u32, 0u32, 300u32, 300u32);
        let cropped_img = crop(&mut img, x, y, width, height);

        assert_eq!(cropped_img.width(), width);
        assert_eq!(cropped_img.height(), height);
    }
}
