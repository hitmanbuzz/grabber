use reqwest::blocking::get;

/// Use for grabbing chapter pages from the image url
/// 
/// url = specific chapter url for the specific comic
pub fn get_url(url: String, domain: String) -> Result<Vec<String>, Box<(dyn std::error::Error + 'static)>> {
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
pub fn get_comic_chapters(comic_url: String, domain: String) -> Result<Vec<String>, Box<(dyn std::error::Error + 'static)>> {
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