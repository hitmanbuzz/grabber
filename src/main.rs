mod source;
mod image;

use std::fs;
use std::io::{self, stdout, Write};
use crate::source::readm::readm;
use crate::image::join_images;

fn main() -> std::io::Result<()> {
    let dir_path = String::from("comic/millennial_cultivator/chapter_0/");
    let mut full_images_path = vec![];

    // Read the contents of the directory
    let entries = fs::read_dir(dir_path.clone())?;

    // Iterate over the directory entries
    for entry in entries {
        // Unwrap the entry
        let entry = entry?;
        // Get the file name as a string
        let file_name = entry.file_name().into_string().unwrap();
        // Print the file name
        full_images_path.push(dir_path.clone() + &file_name);
    }
    join_images(full_images_path, "joined.jpg".to_owned());

    Ok(())
}

fn cli_options() -> String {
    print!("
[1] READM (readm.today)
[?] More coming soon

[#] Choose: ");
    stdout().flush().unwrap();
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("failed to take input for option");
    let option = option.trim();
    return option.to_string();
}