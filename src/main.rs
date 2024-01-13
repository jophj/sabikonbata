use rayon::iter::{ParallelIterator, IntoParallelIterator};
use walkdir::WalkDir;
use std::{env, ffi::OsStr, path::{Path, PathBuf}};

mod image_actions;
mod misc;

use image_actions::find_empty_space::find_margins;

pub fn load_image(path: &Path) -> image::DynamicImage {
    image::open(path).expect("Failed to open image")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: sabikonbata <input_directory> <output_directory>");
        return;
    }

    let input_directory = args[1].as_str();
    let output_directory = args[2].as_str();
    
    let images_result = find_jpg_images(input_directory);
    let images = images_result.expect("Failed to list images");
    println!("Found {} images", images.len());
    
    println!("Creating output directory tree {}", output_directory);
    misc::make_directories(&images, output_directory);

    println!("Start processing");
    images.into_par_iter().for_each(|file| {
        println!("Processing {}", file.display());
        let mut img = load_image(&file);
        let margins = find_margins(&mut img, 100.0);
        let cropped_img = image_actions::crop(
            &mut img,
            margins.0 .0,
            margins.0 .1,
            margins.1 .0 - margins.0 .0,
            margins.1 .1 - margins.0 .1,
        );
        // save the image to a new file path, keeping the original structure
        let new_image_path = misc::swap_path_root(&file, output_directory);
        println!("Saving image to {}", new_image_path.display());
        cropped_img
            .to_image()
            .save(new_image_path)
            .expect("Failed to save image");
    });
}

fn find_jpg_images<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut jpg_images = Vec::new();

    for entry in WalkDir::new(path.as_ref()).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("jpg")) {
            jpg_images.push(path.to_path_buf());
        }
    }

    Ok(jpg_images)
}
