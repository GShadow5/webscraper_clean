use std::path::PathBuf;

use crate::{image_data::ImageBox, named, Config};
use chrono::DateTime;
use json::{array, object, JsonValue};
use log::{debug, trace, warn};

/*
The json code is pretty much all just converting structs to json and back again.
*/

#[named]
pub fn make_config_json(config: Config) -> JsonValue {
    debug!(target: "w10s_webscraper", "{} - Converting config data to json", function_name!());
    let download = config.download;
    let url = &config.url;
    let scan = config.scan;
    let image_directory = &config.image_directory;
    let image_directory = image_directory.to_str().unwrap();
    let test = config.test;
    let verbosity = config.verbosity;


    let json = object! {
        "download": download,
        "url": url.to_string(),
        "scan": scan,
        "image_directory": image_directory,
        "test": test,
        "verbosity": verbosity,
    };
    json
}

#[named]
pub fn parse_config_json(json: JsonValue) -> Config {
    debug!(target: "w10s_webscraper", "{} - Parsing config json", function_name!());
    let download = json["download"].as_bool().unwrap();
    let url = json["url"].to_string();
    let scan = json["scan"].as_bool().unwrap();
    let image_directory = json["image_directory"].to_string();
    let test = json["test"].as_bool().unwrap();
    let verbosity = json["verbosity"].as_u8().unwrap();

    let config = Config {
        download,
        url,
        scan,
        image_directory: PathBuf::from(image_directory),
        test,
        verbosity,
    };
    debug!(target: "w10s_webscraper", "{} - Finished parsing json", function_name!());
    config
}

#[named]
pub fn make_image_json(images: Vec<ImageBox>) -> JsonValue {
    trace!(target: "w10s_webscraper", "{} - Converting image data to json", function_name!());
    let mut json = object! {
        "info": r#"A file with "blacklist" = "true" means that the image entry will remain in the database, but will not be downloaded. This allows you to delete a photo and not download it again. Blacklisted images will keep their entries when the image is absent and you run a file scan."#,
        "images": array![]
    };
    for image in images {
        let image_json = object! {
            "hash": image.hash,
            "date_added": image.date.to_rfc2822(),
            "url": image.url,
            "title": image.title,
            "blacklisted": image.blacklisted,
        };
        json["images"].push(image_json).unwrap();
    }
    trace!(target: "w10s_webscraper", "{} - Finished conversion", function_name!());
    json
}

#[named]
pub fn parse_image_json(json: JsonValue) -> Vec<ImageBox> {
    debug!(target: "w10s_webscraper", "{} - Parsing image json", function_name!());
    let mut images: Vec<ImageBox> = Vec::new();
    for image in json["images"].members() {
        let image_box = ImageBox {
            url: image["url"].to_string(),
            date: DateTime::from(
                match DateTime::parse_from_rfc2822(image["date_added"].to_string().as_str()) {
                    Ok(date) => date,
                    Err(error) => {
                        warn!(
                            target: "w10s_webscraper",
                            "{} - Error parsing date, defaulting to unix 0: {}",
                            function_name!(),
                            error
                        );
                        DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 +0000").unwrap()
                    }
                },
            ),
            title: image["title"].to_string(),
            hash: image["hash"].to_string(),
            blacklisted: image["blacklisted"].as_bool().unwrap(),
        };
        images.push(image_box);
    }
    debug!(target: "w10s_webscraper", "{} - Finished parsing json", function_name!());
    images
}
