use std::fs;

use image::ImageBuffer;
use reqwest::blocking::get;

pub fn readm() {
    let domain = String::from("https://readm.today");
    let comic_url = String::from("https://readm.today/manga/the-all-knowing-cultivator");
    let comic_title: Vec<&str> = comic_url.split("/manga/").collect();
    let comic_title = comic_title[1].replace('-', "_");
    let parent_dir = "comic";
    let comic_dir = comic_title;
    let mut chapter_counter = 0;
    let mut page_counter = 1;

    // println!("{:#?}",result(domain, comic_url));
    for i in readm_result(domain, comic_url) {
        if let Ok(uri) = i {
            let comic_chapter_dir = format!("{}/{}/chapter_{}", parent_dir, comic_dir, chapter_counter);
            let _ = fs::create_dir_all(comic_chapter_dir.clone());
            for url in uri {
                // println!("{}", url);
                let comic_chapter_pages = format!("{}/page_{}.jpg", comic_chapter_dir, page_counter);
                let _ = get_image(comic_chapter_pages.clone(), url);
                println!("{}", comic_chapter_pages.clone());
                page_counter += 1;
            }
            chapter_counter += 1;
            page_counter = 1;
        }
    }
}


/// Use for checking better response type
fn readm_result(domain: String, comic_url: String) -> Vec<Result<Vec<String>, Box<(dyn std::error::Error + 'static)>>> {
    let response = get_comic_chapters(comic_url, domain.clone());
    let mut chapter_url = Vec::new();
    if let Ok(u) = response {
        for uri in u {
            let result = get_url(uri, domain.clone());
            chapter_url.push(result);
        }
    }
    // return all the pages from each chapter of the comic
    return chapter_url;
}


/// Download the comic pages as images
/// 
/// file_name = the file name for the downloaded images (pages from the chapter)
/// 
/// img_url = url for the pages image (.jpg)
fn get_image(file_name: String, img_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let img_bytes = reqwest::blocking::get(img_url)?
    .bytes()?;

    let image = image::load_from_memory(&img_bytes)?;
    image.save(&file_name)?;
    Ok(())
}

/// Use for grabbing chapter pages from the image url
/// 
/// url = specific chapter url for the specific comic
fn get_url(url: String, domain: String) -> Result<Vec<String>, Box<(dyn std::error::Error + 'static)>> {
    // Send a GET request to the URL
    let response = get(url)?;
    let mut url_container = Vec::new();

    // Check if the request was successful
    if response.status().is_success() {
        // Get the text of the response
        let body = response.text()?;

        for target in body.lines() {
            let sub_strings: Vec<&str> = target.split('.').collect();

            // Specific string to check for
            let specific_string = "v=12";

            // Check if any of the split substrings contain the specific string
            let contains_specific_string = sub_strings.iter().any(|&s| s.contains(specific_string));

            // Print the result
            if contains_specific_string {
                for i in sub_strings[0].lines() {
                    let i_left: Vec<&str> = i.split('"').collect();
                    url_container.push(domain.clone() + i_left[1] + ".jpg");
                }
            }
        }
    }
    else {
        println!("{:?}", response);
    }
    return Ok(url_container);
}


/// Grab all the chapters url for the specific comic
/// 
/// comic_url =  url for the comic (eg. https://readm.today/manga/magic-emperor)
/// 
/// domain = url for the main site (eg. https://readm.today)
fn get_comic_chapters(comic_url: String, domain: String) -> Result<Vec<String>, Box<(dyn std::error::Error + 'static)>> {
    // Send a GET request to the URL
    let response = get(comic_url)?;
    let mut comic_url_container: Vec<String> = Vec::new();

    // Check if the request was successful
    if response.status().is_success() {
        // Get the text of the response
        let body = response.text()?;
        let main_splitter = "/manga/";
        
        for target in body.lines() {
            let sub_strings: Vec<&str> = target.split(&main_splitter).collect();

            // Specific string to check for
            let specific_string = "all-pages";

            let contains_specific_string = sub_strings.iter().any(|&s| s.contains(specific_string));

            if contains_specific_string {
                let i_left = sub_strings[1];
                let i_left = i_left.replace('"', "");
                for i in i_left.lines() {
                    let left_index: Vec<&str> = i.split(specific_string).collect();
                    comic_url_container.push(domain.clone() + &main_splitter + left_index[0] + specific_string);
                }
            }
        }
    }
    if !comic_url_container.is_empty() {
        comic_url_container.remove(0);
        comic_url_container.reverse();
    }
    return Ok(comic_url_container)
}

/// Merging plan will not be used for right now as it is not needed
///
/// It is not build yet
#[allow(dead_code)]
fn merge_image(url: String, domain: String) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
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