use std::path::PathBuf;

pub fn make_directories(input_directory: &str, output_directory: &str, images: &Vec<PathBuf>) {
    for image in images {
        let new_image_path = swap_path_prefix(input_directory, output_directory, image);
        std::fs::create_dir_all(new_image_path.parent().unwrap()).unwrap();
    }
}

// Transform a PathBuf like ./images/2020/01/01/image.jpg to ./cropped/2020/01/01/image.jpg
pub fn swap_path_prefix(prefix: &str, new_prefix: &str, path: &PathBuf) -> PathBuf {
    let new_path = path.strip_prefix(prefix).unwrap();

    new_prefix
        .parse::<PathBuf>()
        .unwrap()
        .join(new_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_current_folder_should_swap_path() {
        let path = PathBuf::from("./images/2020/01/01/image.jpg");
        let swapped_path = swap_path_prefix("./images", "./cropped", &path);
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/2020/01/01/image.jpg")
        );
    }

    #[test]
    fn with_parent_folder_should_swap_path() {
        let path = PathBuf::from("../images/2020/01/01/image.jpg");
        let swapped_path = swap_path_prefix("../images", "./cropped", &path);
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/2020/01/01/image.jpg")
        );
    }

    #[test]
    fn with_parent_subfolder_should_swap_path() {
        let path = PathBuf::from("../memes/images/2020/01/01/image.jpg");
        let swapped_path = swap_path_prefix("../memes/images", "./cropped", &path);
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/2020/01/01/image.jpg")
        );
    }

    #[test]
    fn with_absolute_path_should_swap_path() {
        let path = PathBuf::from("/users/jop/Desktop/images/2020/01/01/image.jpg");
        let swapped_path = swap_path_prefix("/users/jop/Desktop/images", "./cropped", &path);
        assert_eq!(
            swapped_path,
            PathBuf::from("./cropped/2020/01/01/image.jpg")
        );
    }
}
