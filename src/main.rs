mod url;
mod image;

use std::fs;

use url::{get_url, get_comic_chapters};
use image::get_image;

fn main() -> std::io::Result<()> {
    let domain = String::from("https://readm.today");
    let comic_url = String::from("https://readm.today/manga/the-all-knowing-cultivator");
    let comic_title: Vec<&str> = comic_url.split("/manga/").collect();
    let comic_title = comic_title[1].replace('-', "_");
    let parent_dir = "comic";
    let comic_dir = comic_title;
    let mut chapter_counter = 0;
    let mut page_counter = 1;

    // println!("{:#?}",result(domain, comic_url));
    for i in result(domain, comic_url) {
        if let Ok(uri) = i {
            let comic_chapter_dir = format!("{}/{}/chapter_{}", parent_dir, comic_dir, chapter_counter);
            let _ = fs::create_dir_all(comic_chapter_dir.clone());
            for url in uri {
                // println!("{}", url);
                let comic_chapter_pages = format!("{}/page_{}.jpg", comic_chapter_dir, page_counter);
                let save_pages= get_image(comic_chapter_pages, url);
                println!("{:#?}", save_pages);
                page_counter += 1;
            }
            chapter_counter += 1;
            page_counter = 1;
        }
    }

    Ok(())
}

/// Use for checking better response type
fn result(domain: String, comic_url: String) -> Vec<Result<Vec<String>, Box<(dyn std::error::Error + 'static)>>> {
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