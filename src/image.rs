use image::ImageBuffer;
use crate::url::get_url;

/// Download the comic pages as images
/// 
/// file_name = the file name for the downloaded images (pages from the chapter)
/// 
/// img_url = url for the pages image (.jpg)
pub fn get_image(file_name: String, img_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let img_bytes = reqwest::blocking::get(img_url)?
    .bytes()?;

    let image = image::load_from_memory(&img_bytes)?;
    image.save(&file_name)?;
    Ok(())
}

/// Merging plan will not be used for right now as it is not needed
///
/// It is not build yet
#[allow(dead_code)]
pub fn merge_image(url: String, domain: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let call_url: Vec<String> = get_url(url, domain).expect("Couldn't fetch the url");
    let mut images = Vec::new();
    for mut i in call_url {
        i = i.replace("/", "\\");
        let image = image::open(i)?;
        images.push(image);
    }

    // Get dimensions of the merged image
    let width: u32 = images.iter().map(|img| img.width()).sum();
    let height: u32 = images.iter().map(|img| img.height()).max().unwrap_or(0);

    // Create a new image buffer to hold the merged images
    let mut merged_image = ImageBuffer::new(width, height);

    // Paste images onto the merged image buffer
    let mut offset_x = 0;
    for img in images {
        image::imageops::overlay(&mut merged_image, &img, (offset_x as i32).into(), 0);
        offset_x += img.width();
    }

    // Save the merged image
    merged_image.save("chapter.jpg")?;

    println!("Images merged successfully.");

    Ok(())
}