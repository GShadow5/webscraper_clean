use crate::ImageBox;
use crate::CONFIG;
pub mod fetch;
pub mod html;
use crate::percentage::Percentage;
use crate::{est_time, named};
use log::{error, info};

#[named]
pub fn get_page_links(url: &str, old_images: &Vec<ImageBox>) -> Vec<String> {
    info!(target: "w10s_webscraper", "fn {} - Collecting page links", function_name!());
    let html = fetch::fetch_html(url).unwrap_or_else(|error| {
        panic!("Problem fetching primary page: {}", error);
    });
    let mut page_links: Vec<String> = html::extract_image_page_links(&html, &old_images);
    if CONFIG.test == false {
        // if debug is false, then we want to get all of the pages
        let page_count = html::extract_page_count(html);
        let mut percent = Percentage::new(page_count as usize);
        info!(target: "w10s_webscraper", "fn {} - Scanning {} pages for links", function_name!(), page_count);
        info!(target: "w10s_webscraper", "fn {} - Estimated time: {}", function_name!(), est_time(page_count as usize));
        let mut fully_skipped_page_count = 0;
        for i in 2..page_count {
            let url = format!("{}page/{}/", url, i);
            let html = match fetch::fetch_html(&url) {
                Ok(html) => html,
                Err(error) => {
                    error!(target: "w10s_webscraper", "fn {} - Problem fetching page {}: {}", function_name!(), url, error);
                    continue;
                }
            };
            let mut new_links: Vec<String> =
                html::extract_image_page_links(&html, &old_images).to_vec();

            // If we get no new links five pages in a row, then we can skip the rest of the pages
            if new_links.len() == 0 {
                fully_skipped_page_count += 1;
            }
            if new_links.len() > 0 {
                fully_skipped_page_count = 0;
            }
            if fully_skipped_page_count > 5 {
                info!(target: "w10s_webscraper", "fn {} - No new images found for five pages, stopping", function_name!());
                break;
            }

            page_links.append(&mut new_links);
            percent.update(function_name!());
        }
    }

    page_links
}

#[named]
pub fn get_image_data(urls: Vec<String>) -> Vec<ImageBox> {
    let mut percent = Percentage::new(urls.len());
    info!(target: "w10s_webscraper", "fn {} - Collecting data on {} images", function_name!(), urls.len());
    info!(target: "w10s_webscraper", "fn {} - Estimated time: {}", function_name!(), est_time(urls.len()));
    let mut images: Vec<ImageBox> = Vec::new();
    for url in urls {
        let html = match fetch::fetch_html(&url) {
            Ok(html) => html,
            Err(error) => {
                error!(target: "w10s_webscraper", "fn {} - Problem fetching page {}: {}", function_name!(), url, error);
                continue;
            }
        };
        let image_link = html::extract_image_url(&html);
        let image_title = html::extract_image_title(&html);
        let image_date = html::extract_image_date(&html);
        let image_hash = url.split("/").last().unwrap().to_string();
        let image = ImageBox {
            url: image_link,
            date: image_date,
            title: image_title,
            hash: image_hash,
            blacklisted: false,
        };
        images.push(image);
        percent.update(function_name!());
    }
    images
}
