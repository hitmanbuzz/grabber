mod url;
mod image;

use url::{get_url, get_comic_chapters};
use image::get_image;

fn main() {
    let domain = String::from("https://readm.today");
    let comic_url = String::from("https://readm.today/manga/the-all-knowing-cultivator");
    println!("{:#?}",result(domain, comic_url));
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
    return chapter_url;
}
