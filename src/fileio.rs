use chrono::Local;
use filetime_creation::{set_file_ctime, FileTime};
use json::JsonValue;
use log::{debug, error, info, trace};
use md5;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

use crate::named;
use crate::percentage::Percentage;
use crate::web::fetch::fetch_image_bytes;
use crate::ImageBox;

#[named]
pub fn write_image(image: &ImageBox, image_directory: &Path) -> Result<(), Error> {
    // Join the image_directory path with the image title and .jpg
    let image_path = image_directory.join(&image.title).with_extension(".jpg");
    // Create the image file
    let mut out = File::create(&image_path)?;
    // Fetch the image bytes
    let mut content = match fetch_image_bytes(&image.url) {
        Ok(content) => content,
        Err(error) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Error fetching image bytes from {}: {}", &image.url, error),
            ));
        }
    };
    // Write the image bytes to the image file
    out.write_all(&mut content)?;

    // Next we need to set the creation date of the file to the date of the image
    let image_time = FileTime::from_unix_time(image.date.timestamp(), 0);
    set_file_ctime(&image_path, image_time)?;
    trace!(target: "w10s_webscraper", "{} - Image {} written", function_name!(), image.title);
    Ok(())
}

#[named]
pub fn read_images(image_directory: &Path) -> Vec<ImageBox> {
    // reads the image directory and returns a vector of ImageBox structs with hashes from the actual images
    let mut images: Vec<ImageBox> = Vec::new();
    // Iterate over the files in the image directory
    let files = fs::read_dir(&image_directory)
        .unwrap_or_else(|error| {
            error!(target: "w10s_webscraper", "fn {} - Error reading image directory: {}", function_name!(), error);
            panic!(
                "{} - Error reading image directory: {}",
                function_name!(),
                error
            )
        })
        .collect::<Vec<_>>();
    let mut percent = Percentage::new(files.len());
    info!(target: "w10s_webscraper", "fn {} - Reading {} files", function_name!(), files.len());
    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(error) => {
                error!(target: "w10s_webscraper", "fn {} - Error reading file, skipping: {}", function_name!(), error);
                continue;
            }
        };
        // If the file is a jpg, read the hash from the file and add it to the vector
        let jpg = match file.path().extension() {
            Some(str) => {
                if str == "jpg" {
                    true
                } else {
                    trace!(target: "w10s_webscraper", "fn {} - File is not a jpg, skipping: {}", function_name!(), file.path().display());
                    false
                }
            }
            _ => {
                trace!(target: "w10s_webscraper", "fn {} - File has no extension?, skipping: {}", function_name!(), file.path().display());
                false
            }
        };
        if jpg {
            let hash = md5::compute(std::fs::read(file.path()).unwrap());
            let image_box = ImageBox {
                url: "scanned".to_string(),
                date: Local::now(),
                title: file.file_name().into_string().unwrap(),
                hash: format!("{:x}", hash),
                blacklisted: false,
            };
            images.push(image_box);
            trace!(
                target: "w10s_webscraper",
                "fn {} - Image {} read",
                function_name!(),
                &file.file_name().into_string().unwrap()
            );
        }
        percent.update(function_name!());
    }
    images
}

#[named]
pub fn read_json(image_directory: &Path) -> Result<JsonValue, Error> {
    let path = image_directory.join("hashes.json");
    // Read the json from the file and return it
    trace!(
        target: "w10s_webscraper",
        "fn {} - Reading json file, expect confirmation",
        function_name!()
    );
    let mut file = File::open(path)?;
    trace!(target: "w10s_webscraper", "fn {} - json file read", function_name!());

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let json = json::parse(&buf).unwrap();
    debug!(target: "w10s_webscraper", "fn {} - Loaded json file", function_name!());
    Ok(json)
}

#[named]
pub fn write_json(path: &Path, json: JsonValue) {
    let json = json.pretty(2);
    let path = path.join("hashes.json");
    // Create file, and overwrite it if it exists
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(error) => panic!("{} - Error creating json file: {}", function_name!(), error),
    };
    match file.write_all(json.as_bytes()) {
        Ok(_) => (),
        Err(error) => panic!("{} - Error writing json file: {}", function_name!(), error),
    };
    debug!(target: "w10s_webscraper", "fn {} - Wrote json file", function_name!());
}
