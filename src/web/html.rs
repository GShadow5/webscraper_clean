use chrono::{DateTime, Local};
use log::{debug, error, warn};
use scraper::{Html, Selector};

use crate::{named, ImageBox};

#[named]
pub fn extract_page_count(html: Html) -> i32 {
    let mut page_count: i32 = 0;
    // select only the links that are page numbers
    let selector = Selector::parse(r#"a[class="page-numbers"]"#).unwrap();
    let links = html.select(&selector);
    // iterate over the links and find the highest page number
    for link in links {
        let href = link.value().attr("href").unwrap();
        // get the last element of the link, which is the page number
        let raw = href.split("/").collect::<Vec<&str>>();
        let last = raw.last().unwrap();
        let last = last.parse::<i32>().unwrap(); // cast the last element to an i32
        if last > page_count {
            page_count = last;
        }
    }
    debug!(target: "w10s_webscraper", "fn {} - Extracted page count: {}", function_name!(), page_count);
    page_count
}

#[named]
pub fn extract_image_page_links(html: &Html, old_images: &Vec<ImageBox>) -> Vec<String> {
    let mut page_links: Vec<String> = Vec::new();
    let selector = Selector::parse(r#"a[href]"#).unwrap();
    let links = html.select(&selector);
    for link in links {
        // get the href attribute
        let href: String = link.value().attr("href").unwrap().to_string();
        // get the hash in the link
        let hash: String = href
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        // if the link is an image, and it is not a comment link, and it is not a duplicate, then add it to the list
        let image = old_images.iter().find(|&x| x.hash == hash);
        if href.contains("/images/")
            && href.contains("#respond") == false
            && href.contains("#comments") == false
            && page_links.contains(&href) == false
            && image == None
        {
            page_links.push(href.to_string());
        }
    }
    debug!(target: "w10s_webscraper", "fn {} - Extracted {} links: {:?}", function_name!(), page_links.len(), page_links);
    page_links
}

pub fn extract_image_url(html: &Html) -> String {
    // construct a CSS selector that will grab all of the image tags
    // This selector is not the html snippets themselves, but rather an object that knows how to select them
    let selector = Selector::parse("img").unwrap();
    // use the selector to find all img tags in the document
    let images = html.select(&selector);
    // iterate over the elements (references to tags) that the selector found, and assign the correct one to the output variable
    let mut output: String = String::new();
    for image in images {
        // get the raw src attribute of the image tag
        let src = image.value().attr("src").unwrap();
        // output the src attribute if it contains "jpg" and "wp-content/uploads/" and "1024x576"
        if src.contains("jpg") && src.contains("wp-content/uploads/") && src.contains("1024x576") {
            //println!("{}", src);
            // split the src attribute into a vector of strings, using the "-" character as the delimiter
            let tempvec = src.split("-").collect::<Vec<&str>>();
            // create a new string, and push the first two elements of the vector into it, separated by a "-", and add ".jpg" to the end
            let mut temp_s = String::new();
            temp_s.push_str(tempvec[0]);
            // this keeps the '-' in ".com/wp-content/upl"
            temp_s.push_str("-");
            temp_s.push_str(tempvec[1]);
            temp_s.push_str(".jpg");
            output = temp_s;
            //print!("{}", output)
        }
    }
    output
}

#[named]
pub fn extract_image_date(html: &Html) -> DateTime<Local> {
    let selector = Selector::parse(r#"span[class="date"]"#).unwrap();
    let html_dates = html.select(&selector);
    let mut dates: Vec<String> = Vec::new();
    for date in html_dates {
        let date = date.text().collect::<Vec<_>>();
        dates.push(date[0].to_string())
    }
    if dates.len() > 1 {
        warn!(target: "w10s_webscraper", "{} - More than one date found on page", function_name!());
    }
    // date comes out of the html as "2020-01-01", but we need to add the time and timezone to it
    // so we can parse it into a DateTime object
    let mut datetime: String = dates[0].to_string();
    datetime.push_str("  12:00:00 -0500");
    let datetime = match DateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S %z") {
        Ok(datetime) => datetime.with_timezone(&Local),
        Err(e) => {
            error!(
                target: "w10s_webscraper",
                "{} - Error parsing date, using local now: {}",
                function_name!(),
                e
            );
            Local::now()
        }
    };
    datetime
}

#[named]
pub fn extract_image_title(html: &Html) -> String {
    let selector = Selector::parse(r#"title"#).unwrap();
    let titles = html.select(&selector);
    let mut output: Vec<String> = Vec::new();
    for title in titles {
        let title = title.text().collect::<Vec<_>>();
        output.push(title[0].to_string())
    }
    if output.len() > 1 {
        warn!(
            target: "w10s_webscraper",
            "{} - More than one title found. Using the first one ({})",
            function_name!(),
            output[0]
        );
    }

    output[0]
        .split(" | ")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string()
}
