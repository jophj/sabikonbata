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

    // list all files in the directory from args1 with the standard library
    let images = find_jpg_images(args[1].as_str());

    images.unwrap().into_par_iter().for_each(|file| {
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
        let latest_components = file.components().take(file.components().count() - 3);
        println!("Latest components: {:?}", file.components().collect::<PathBuf>());
        let new_image_path = "./cropped/".to_owned() + &latest_components.collect::<PathBuf>().to_str().unwrap() + "/" + file.file_name().unwrap().to_str().unwrap();
        println!("Saving image to {}", new_image_path);
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
