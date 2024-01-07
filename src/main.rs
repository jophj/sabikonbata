use rayon::iter::{Filter, ParallelBridge, ParallelIterator};
use std::{env, fs};

use image_actions::find_empty_space::find_margins;

mod image_actions;

pub fn load_image(path: &str) -> image::DynamicImage {
    image::open(path).expect("Failed to open image")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // list all files in the directory from args1 with the standard library
    let images = fs::read_dir(&args[1])
        .unwrap()
        .filter(|entry| {
            let binding = entry.as_ref().unwrap().path();

            let file_name = binding.to_str().unwrap();

            file_name.ends_with(".png") || file_name.ends_with(".jpg")
        })
        .into_iter();

    images.par_bridge().for_each(|file| {
        println!("Processing {}", file.as_ref().unwrap().path().display());
        let mut img = load_image(&file.as_ref().unwrap().path().display().to_string());
        let margins = find_margins(&mut img, 100.0);
        let cropped_img = image_actions::crop(
            &mut img,
            margins.0 .0,
            margins.0 .1,
            margins.1 .0 - margins.0 .0,
            margins.1 .1 - margins.0 .1,
        );
        let new_image_path = "./cropped/".to_owned() + &file.unwrap().path().display().to_string();
        println!("Saving image to {}", new_image_path);
        cropped_img
            .to_image()
            .save(new_image_path)
            .expect("Failed to save image");
    });

    // for file in images {
    //     println!("Processing {}", file.as_ref().unwrap().path().display());
    //     let mut img = load_image(&file.as_ref().unwrap().path().display().to_string());
    //     let margins = find_margins(&mut img, 100.0);
    //     let cropped_img = image_actions::crop(&mut img, margins.0.0, margins.0.1, margins.1.0 - margins.0.0, margins.1.1 - margins.0.1);
    //     let new_image_path = "./cropped/".to_owned() + &file.unwrap().path().display().to_string();
    //     println!("Saving image to {}", new_image_path);
    //     cropped_img.to_image().save(new_image_path).expect("Failed to save image");
    // }
}
