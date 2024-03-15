mod url;
mod image;

use url::{get_url, get_comic_chapters};
use image::get_image;

fn main() {
    let domain = String::from("https://readm.today");
    let comic_url = String::from("https://readm.today/manga/invincible-at-the-start");
    let response = get_comic_chapters(comic_url, domain);
    println!("{:#?}", response);
}

fn result(domain: String) -> bool {
    let url = String::from("https://readm.today/manga/after-opening-my-eyes-my-disciples-became-the-great-villainous-empresses/1/all-pages");
    let mut page_counter = 1;
    let result = get_url(url, domain);
    
    if let Ok(i) = result {
        for iu in i {
            let path = format!("image\\p{}.jpg", page_counter);
            let img_url = iu.trim_end().to_owned();
            let response = get_image(path, img_url);
            page_counter += 1;
            println!("{:#?}", response);
        }
        return true;
    }
    return false;
}
