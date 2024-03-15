use reqwest::blocking::get;

pub fn get_url(url: String, domain: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Send a GET request to the URL
    let response = get(url)?;
    let mut url_container = vec![];

    // Check if the request was successful
    if response.status().is_success() {
        // Get the text of the response
        let body = response.text()?;

        for target in body.lines() {
            let substrings: Vec<&str> = target.split('.').collect();

            // Specific string to check for
            let specific_string = "v=12";

            // Check if any of the split substrings contain the specific string
            let contains_specific_string = substrings.iter().any(|&s| s.contains(specific_string));

            // Print the result
            if contains_specific_string {
                for i in substrings[0].lines() {
                    let i_left: Vec<&str> = i.split('"').collect();
                    url_container.push(domain.to_owned() + i_left[1] + ".jpg");
                }
            }
        }
    }
    return Ok(url_container);
}