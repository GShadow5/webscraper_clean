use crate::{est_time, fileio, named, percentage, web, ImageBox, CONFIG};
use log::{error, info, warn};

#[named]
pub fn download_images(old_images: &Vec<ImageBox>) -> Vec<ImageBox> {
    info!(target: "w10s_webscraper", "fn {} - Downloading images", function_name!());
    // Get the page links from the website
    let page_links = web::get_page_links(&CONFIG.url, &old_images);
    // Get the image data from each page of the website, keeping data on only the new images
    let images: Vec<ImageBox> = web::get_image_data(page_links);

    let mut percent = percentage::Percentage::new(images.len() as usize);
    info!(target: "w10s_webscraper", "fn {} - Downloading and writing {} images", function_name!(), images.len());
    info!(target: "w10s_webscraper", "fn {} - Estimated time: {}", function_name!(), est_time(images.len()));
    // Download and write each new image
    for image in &images {
        match fileio::write_image(&image, &CONFIG.image_directory) {
            Ok(_) => {}
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::AlreadyExists => {
                        warn!(target: "w10s_webscraper", "fn {} - Image already exists, skipping: {}", function_name!(), image.hash);
                    }
                    std::io::ErrorKind::Other => {
                        warn!(target: "w10s_webscraper", "fn {} - Error fetching image bytes, skipping: {}", function_name!(), error);
                    }
                    _ => {
                        error!(target: "w10s_webscraper", "fn {} - Error writing image, skipping: {}", function_name!(), error);
                    } // No need "continue", as the image is not written
                }
            }
        };
        percent.update(function_name!());
    }
    images
}
