use reqwest::blocking::get;

pub fn get_url(url: String, domain: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

pub fn get_comic_chapters(comic_url: String, domain: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
                // for i in sub_strings[0].lines() {
                //     let i_left: Vec<&str> = i.split('"').collect();
                //     // comic_url_container.push(i_left[1].to_owned());
                //     comic_url_container.push(i_left[0].to_owned());
                // }
                let i_left = sub_strings[1];
                let i_left = i_left.replace('"', "");
                for i in i_left.lines() {
                    let left_index: Vec<&str> = i.split(specific_string).collect();
                    comic_url_container.push(domain.clone() + &main_splitter + left_index[0] + specific_string);
                }
            }
        }
    }
    return Ok(comic_url_container)
}