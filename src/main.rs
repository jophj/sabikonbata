use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::
    env
;

use crate::image_actions::{find_margins::find_margins, crop::crop};

mod misc;
mod image_actions;
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: sabikonbata <input_directory> <output_directory>");
        return;
    }

    let input_directory = args[1].as_str();
    let output_directory = args[2].as_str();

    let images_result = filesystem::find_images(input_directory);
    let images = images_result.expect("Failed to list images");
    println!("Found {} images", images.len());

    println!("Creating output directory tree {}", output_directory);
    misc::make_directories(input_directory, output_directory, &images);

    println!("Start processing");
    images.into_par_iter().for_each(|file| {
        println!("Processing {}", file.display());
        let mut img = filesystem::load_image(&file);
        let margins = find_margins(&mut img, 100.0);
        let cropped_img = crop(
            &mut img,
            margins.0 .0,
            margins.0 .1,
            margins.1 .0 - margins.0 .0,
            margins.1 .1 - margins.0 .1,
        );
        // save the image to a new file path, keeping the original structure
        let new_image_path = misc::swap_path_prefix(input_directory, output_directory, &file);
        println!("Saving image to {}", new_image_path.display());
        cropped_img
            .to_image()
            .save(new_image_path)
            .expect("Failed to save image");
    });
}

