use crate::named;
use lazy_static::lazy_static;
use log::{debug, trace};
use reqwest::Error;
use scraper::Html;
use std::sync::Mutex;
use std::time::Instant;

/*
   do_throttled_request is heavily inspired by https://github.com/gregstoll/rust-scraping, but I've made a lot of changes
*/

lazy_static! {
    static ref LAST_REQUEST_MUTEX: Mutex<Option<Instant>> = Mutex::new(None);
    static ref REQUEST_DELAY: std::time::Duration = std::time::Duration::from_millis(500);
}

pub fn do_throttled_request(url: &str) -> Result<reqwest::blocking::Response, Error> {
    fn delay() {
        let mut last_request_mutex = LAST_REQUEST_MUTEX.lock().unwrap();
        let last_request = last_request_mutex.take();
        //let now = Instant::now();

        if let Some(last_request) = last_request {
            let duration = last_request.duration_since(last_request);
            if duration < *REQUEST_DELAY {
                std::thread::sleep(*REQUEST_DELAY - duration);
            }
        }
    }

    // First request
    delay();
    let mut resp = reqwest::blocking::get(url);
    LAST_REQUEST_MUTEX.lock().unwrap().replace(Instant::now());
    // Retry 5 times
    if resp.is_err() {
        for i in 0..5 {
            delay();
            resp = reqwest::blocking::get(url);
            LAST_REQUEST_MUTEX.lock().unwrap().replace(Instant::now());

            if resp.is_ok() {
                break;
            }
            if i == 4 {
                return resp; // Return error after 5 tries
            }
        }
    }
    resp
}

#[named]
pub fn fetch_html(url: &str) -> Result<scraper::Html, Error> {
    trace!(target: "w10s_webscraper", "fn {} - Fetching HTML from {}", function_name!(), url);
    let resp = match do_throttled_request(url) {
        Ok(resp) => resp,
        Err(e) => {
            debug!(target: "w10s_webscraper", "fn {} - Error fetching HTML from {}", function_name!(), url);
            return Err(e);
        }
    };

    let html = resp.text().unwrap();
    let html = Html::parse_document(&html);
    Ok(html)
}

pub fn fetch_image_bytes(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let resp = do_throttled_request(url)?;
    let bytes = resp.bytes()?;
    Ok(bytes.to_vec())
}
