use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use ureq::http::HeaderValue;

const DOWNLOAD_TEMPLATE: &str =
    "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

#[derive(Deserialize)]
pub struct ImageBody {
    id: String,
    slug: String,
    urls: Urls,
    user: User,
}

#[derive(Deserialize)]
pub struct Urls {
    full: String
}

#[derive(Deserialize)]
pub struct User {
    name: String
}

pub fn get_random_image(access_key: &str) -> Result<ImageBody, ureq::Error> {
    ureq::get("https://api.unsplash.com/photos/random")
        .query_pairs(vec![("orientation", "landscape")])
        .header("Authorization", format!("Client-ID {}", access_key))
        .call()?
        .body_mut()
        .read_json::<ImageBody>()
}

enum ClientError {
    NoHeader,
    ParsingHeaderError(HeaderValue),
    ParsingNumber(String),
}

pub fn download_image<P: AsRef<Path>>(file_path: P, access_key: &str) -> Result<(), Box<dyn Error>> {
    let response =
        ureq::get("https://images.unsplash.com/photo-1739609579483-00b49437cc45?crop=entropy&cs=srgb&fm=jpg&ixid=M3w3MTQ0MjJ8MHwxfGFsbHwxfHx8fHx8fHwxNzQwNDg0OTA0fA&ixlib=rb-4.0.3&q=85")
            .header("Authorization", format!("Client-ID {}", access_key))
            .call()?;

    if let Err(e) = response.headers().get("Content-Length")
        .ok_or(ClientError::NoHeader)
        .and_then(|header| header.to_str().map_err(|_| ClientError::ParsingHeaderError(header.clone())))
        .and_then(|header_str| {
            u64::from_str_radix(header_str, 10).map_err(|_| ClientError::ParsingNumber(header_str.to_string()))
        })
        .and_then(|length| {
            let bar = ProgressBar::new(!0);
            bar.set_message("Downloading");
            bar.set_style(
                ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
                    .unwrap()
                    .progress_chars("##-"),
            );
            bar.set_length(length);
            Ok(())
        }) {
        eprintln!("Can't show progress bar");
        match e {
            ClientError::NoHeader => eprintln!("Can't find header Content-Length"),
            ClientError::ParsingHeaderError(header) => eprintln!("Can't parse header value: {:?}", header),
            ClientError::ParsingNumber(value) => eprintln!("Can't parse header value into number: {}", value)
        };
    }

    let mut file = File::create(file_path)?;
    std::io::copy(&mut response.into_body().into_reader(), &mut file)?;
    Ok(())
}
