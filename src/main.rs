mod image_actions;
mod filesystem;

fn main() {
    // Load the image (replace "input_image.png" with the path to your image)
    let mut img = filesystem::load_image("input_image.jpg");

    // Define the position and size of the crop (example: top left corner, width 100, height 100)
    let (x, y, width, height) = (0u32, 0u32, 300u32, 300u32);
    
    // Perform the crop operation
    let cropped_img = image_actions::crop(&mut img, x, y, width, height);
        // Save the cropped image (replace "cropped_image.png" with the desired output path)
    cropped_img.to_image().save("cropped_image.png").expect("Failed to save image");
}
