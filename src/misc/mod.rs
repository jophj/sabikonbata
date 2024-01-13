use std::path::PathBuf;

pub fn make_directories(images: &Vec<PathBuf>, output_directory: &str) {
    for image in images {
        let new_image_path = swap_path_root(&image, output_directory);
        std::fs::create_dir_all(new_image_path.parent().unwrap()).unwrap();
    }
}

// Transform a PathBuf like ./images/2020/01/01/image.jpg to ./cropped/2020/01/01/image.jpg
pub fn swap_path_root(path: &PathBuf, new_root: &str) -> PathBuf {
    // skip all current folder components
    let mut to_skip_counter: u8 = 0;
    for component in path.components() {
        if component.as_os_str() == "." {
            to_skip_counter += 1;
        } else {
            break;
        }
    }

    let latest_components = path.components().skip((to_skip_counter + 1) as usize);
    
    let mut new_path = PathBuf::from(new_root);
    for component in latest_components {
        new_path.push(component);
    }

    new_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_current_folder_should_swap_path() {
        let path = PathBuf::from("./images/2020/01/01/image.jpg");
        let swapped_path = swap_path_root(&path, "./cropped");
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/2020/01/01/image.jpg")
        );
    }

    #[test]
    fn with_relative_path_should_swap_path() {
        let path = PathBuf::from("images/2020/01/01/image.jpg");
        let swapped_path = swap_path_root(&path, "cropped");
        assert_eq!(
            swapped_path,
            PathBuf::from("cropped/2020/01/01/image.jpg")
        );
    }

    #[test]
    fn with_absolute_path_should_swap_path() {
        let path = PathBuf::from("/images/2020/01/01/image.jpg");
        let swapped_path = swap_path_root(&path, "./cropped");
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/images/2020/01/01/image.jpg")
        );
    }
}
