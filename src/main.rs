use argparse::Config;
pub use function_name::named;
use json::JsonValue;
use lazy_static::lazy_static;
pub use log::{debug, error, info, trace, warn};
pub use log::{Level, LevelFilter};

mod download;
mod fileio;
mod image_data;
mod logging;
pub mod percentage;
mod scan;
pub use image_data::ImageBox;
pub mod argparse;
mod json_code;
mod web;
use json_code::make_image_json;

lazy_static! {
    static ref CONFIG: Config = argparse::parse_args();
}

#[named]
fn main() {
    logging::initialize_logging();

    info!(target: "w10s_webscraper", "{} - Starting", function_name!());
    trace!(target: "w10s_webscraper", "{} - Beginning of json loading", function_name!());
    // Load json database of existing images in the image directory
    let json = match fileio::read_json(&CONFIG.image_directory) {
        Ok(json) => json,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                warn!(target: "w10s_webscraper", "{} - Json file not found, will create one", function_name!());
                JsonValue::new_object()
            } else {
                error!(target: "w10s_webscraper", "{} - Error reading json file: {}", function_name!(), error);
                panic!("{} - Error reading json file: {}", function_name!(), error);
            }
        }
    };
    // Parse json into a vector of ImageBox structs
    let mut old_images: Vec<ImageBox> = json_code::parse_image_json(json);
    trace!(target: "w10s_webscraper", "{} - End of json loading", function_name!());

    // Create an empty vector of ImageBox structs to hold the new images
    let mut images: Vec<ImageBox> = Vec::new();

    trace!(target: "w10s_webscraper",
        "{} - CONFIG.scan={}, 0: download only, 2: scan only, 1: do both",
        function_name!(),
        CONFIG.scan
    );
    // Determine if we need to download images, scan the image directory, or both
    if CONFIG.scan {
        old_images = scan::scan(&mut old_images);
    }
    if CONFIG.download {
        images = download::download_images(&old_images);
    }

    trace!(target: "w10s_webscraper", "{} - Merging old and new image data", function_name!());
    // Merge the old and new image data
    images.append(&mut old_images);
    trace!(target: "w10s_webscraper", "{} - Writing json", function_name!());

    // Write the new json file
    let json = make_image_json(images);
    trace!(target: "w10s_webscraper", "{} - Writing json to file", function_name!());
    fileio::write_json(&CONFIG.image_directory, json);
    info!(target: "w10s_webscraper", "{} - Finished", function_name!());
}

pub fn est_time(vector: usize) -> String {
    let est_time = (vector as f64 * 5.1) / 60 as f64;
    if est_time > 120.0 {
        format!("{} hours", est_time / 60.0)
    } else {
        format!("{} minutes", est_time)
    }
}
