use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub fn load_image(path: &Path) -> image::DynamicImage {
    image::open(path).expect("Failed to open image")
}

pub fn find_images<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut images = Vec::new();

    for entry in WalkDir::new(path.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("jpg"))
            || path.extension() == Some(OsStr::new("jpeg"))
            || path.extension() == Some(OsStr::new("png"))
        {
            images.push(path.to_path_buf());
        }
    }

    Ok(images)
}
