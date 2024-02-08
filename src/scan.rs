use crate::{named, CONFIG, ImageBox, fileio};
use log::info;

#[named]
pub fn scan(old_images: &mut Vec<ImageBox>) -> Vec<ImageBox> {
    // Scan image directory for existing images
    info!(target: "w10s_webscraper", "fn {} - Scanning directory for existing images", function_name!());
    let scanned_images: Vec<ImageBox> = fileio::read_images(&CONFIG.image_directory);
    if scanned_images.len() == 0 {
        info!(target: "w10s_webscraper", "fn {} - No images found in directory, stopping scan", function_name!());
        return old_images.to_vec();
    }
    let mut indexies_to_remove: Vec<usize> = Vec::new();
    let mut pos: usize = 0;
    // Determine the json entries that are not present in the directory
    for image in &*old_images {
        let mut is_present = false;
        for scanned_image in &scanned_images {
            if image.hash == scanned_image.hash {
                is_present = true;
            }
        }
        if !is_present && image.blacklisted == false {
            indexies_to_remove.push(pos)
        }
        pos += 1;
    }
    // Remove the entries from the json
    // Remove in reverse order to avoid index issues
    indexies_to_remove.reverse();
    for index in indexies_to_remove {
        old_images.remove(index);
    }
    info!(target: "w10s_webscraper", "fn {} - Purged absent images from database", function_name!());

    // Add images that are in the directory, but not in the json
    for image in &scanned_images {
        let mut is_old = false;
        for old_image in &*old_images {
            if image.hash == old_image.hash {
                is_old = true;
                break;
            }
        }

        if !is_old {
            old_images.push(image.clone());
        }
    }
    info!(target: "w10s_webscraper", "fn {} - Added new images to database", function_name!());

    old_images.to_vec()
}