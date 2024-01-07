use std::env;

use image_actions::find_empty_space::find_margins;

mod image_actions;
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Load the image (replace "input_image.png" with the path to your image)
    let mut img = filesystem::load_image(&args[1]);

    // Define the position and size of the crop (example: top left corner, width 100, height 100)
    let margins = find_margins(&mut img, 100.0);
    
    println!("Margins: {:?}", margins);
    // Perform the crop operation
    let cropped_img = image_actions::crop(&mut img, margins.0.0 + 1, margins.0.1 + 1, margins.1.0 - margins.0.0 - 1, margins.1.1 - margins.0.1 - 1);
    // Save the cropped image (replace "cropped_image.png" with the desired output path)
    let new_image_path = "./cropped/".to_owned() + &args[1];
    println!("Saving image to {}", new_image_path);
    cropped_img.to_image().save(new_image_path).expect("Failed to save image");
}
