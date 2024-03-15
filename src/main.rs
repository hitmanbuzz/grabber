mod url;
mod image;

use url::{get_url, get_comic_chapters};
use image::get_image;

fn main() {
    let domain = String::from("https://readm.today");
    let comic_url = String::from("https://readm.today/manga/magic-emperor");
    result(domain, comic_url);
}

/// Use for checking for better response type
fn result(domain: String, comic_url: String) {  
    let mut chapter_counter = 0;
    let response = get_comic_chapters(comic_url, domain.clone());
    let chapter_url: Vec<String> = Vec::new();
    if let Ok(u) = response {
        for uri in u {
            let result = get_url(uri, domain.clone());
            println!("Chapter: {}", chapter_counter);
            println!("{:#?}", result);
            println!("\n");
            chapter_counter += 1;
        }
    }
}
