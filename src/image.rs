use image::{DynamicImage, GenericImage, GenericImageView};
use std::path::Path;

fn join_images_vertically(images: Vec<DynamicImage>) -> DynamicImage {
    // Find the maximum width among all images
    let max_width = images.iter().map(|img| img.width()).max().unwrap_or(0);

    // Calculate the total height required for the stacked image
    let total_height: u32 = images.iter().map(|img| img.height()).sum();

    let mut result = DynamicImage::new_rgb8(max_width, total_height);
    let mut y_offset = 0;

    // Paste each image onto the result image with appropriate offset
    for img in &images {
        let (width, height) = img.dimensions();
        let x_offset = (max_width - width) / 2; // Centering horizontally
        result.copy_from(img, x_offset, y_offset).unwrap();
        y_offset += height;
    }

    result
}

pub fn join_images(images: Vec<String>, final_image_path: String) {
    let image_paths = images;

    let mut images = Vec::new();
    for path in &image_paths {
        let img = image::open(&Path::new(path)).unwrap();
        images.push(img);
    }

    // Join images vertically
    let joined_image = join_images_vertically(images);

    // Save joined image
    joined_image.save(final_image_path).unwrap();
}