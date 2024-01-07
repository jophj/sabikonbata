pub fn load_image(path: &str) -> image::DynamicImage {
  image::open(path).expect("Failed to open image")
}
