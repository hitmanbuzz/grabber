# Buzz Grabber

I made this program for my personal liking

As of now I only made it to grab comics from my favorite site `https://readm.today`

```rust
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

```
*The above code collect all the .jpg files which is the pages for each chapter from the comic and return as `Vec<String>` type if it is succesfull*

`TODO` : Merge page images to make each chapter

    Status: Almost Completed
    Developer: Moirangthem Henthoiba