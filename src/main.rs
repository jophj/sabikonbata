use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{env, io::Write, sync::atomic};

use crate::image_actions::{crop::crop, find_margins::find_margins};

mod filesystem;
mod image_actions;
mod misc;

const PROGRESS_BAR: &[&str; 6] = &["⠧", "⠏", "⠛", "⠹", "⠼", "⠶"];
const PROGRESS_BAR_FULL: &str = "✓";

struct Stats {
    progress: atomic::AtomicU32,
}

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

    let now = std::time::Instant::now();
    let stats = Stats {
        progress: atomic::AtomicU32::new(0),
    };
    let images_count = images.len();
    images.into_par_iter().for_each(|file| {
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
        cropped_img
            .to_image()
            .save(new_image_path)
            .expect("Failed to save image");

        stats.progress.fetch_add(1, atomic::Ordering::SeqCst);
        let progress = stats.progress.load(atomic::Ordering::SeqCst);
        print!(
            "\r\x1b[34m{}\x1b[0m {:4}/{:<4} {:.2} images/second in {:.2} seconds",
            PROGRESS_BAR[progress as usize % PROGRESS_BAR.len()],
            progress,
            images_count,
            progress as f32 / now.elapsed().as_secs_f32(),
            now.elapsed().as_secs_f32()
        );
        let _ = std::io::stdout().flush();
    });
    //clear the progress bar
    print!("\r{}", " ".repeat(64));
    println!(
        "\r\x1b[32m{}\x1b[0m {:4}/{:<4} {:.2} images/second in {:.2} seconds",
        PROGRESS_BAR_FULL,
        images_count,
        images_count,
        images_count as f32 / now.elapsed().as_secs_f32(),
        now.elapsed().as_secs_f32()
    );
}
