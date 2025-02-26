use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use ureq::http::HeaderValue;

const DOWNLOAD_TEMPLATE: &str = "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

#[derive(Deserialize)]
pub struct ImageBody {
    pub id: String,
    pub slug: String,
    pub urls: Urls,
    pub user: User,
}

#[derive(Deserialize)]
pub struct Urls {
    pub full: String,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

pub fn get_random_image(access_key: &str) -> Result<ImageBody, ureq::Error> {
    println!("Getting random image");
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

pub fn download_image<P: AsRef<Path>>(
    download_url: &str,
    file_path: P,
    access_key: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Sending request to: {}", download_url);
    let response = ureq::get(download_url)
        .header("Authorization", format!("Client-ID {}", access_key))
        .call()?;

    let r_length = response
        .headers()
        .get("Content-Length")
        .inspect(|_| println!("Trying to get content length"))
        .ok_or(ClientError::NoHeader)
        .and_then(|header| {
            header
                .to_str()
                .map_err(|_| ClientError::ParsingHeaderError(header.clone()))
        })
        .and_then(|header_str| {
            println!("Try to parse");
            header_str
                .parse::<u64>()
                .map_err(|_| ClientError::ParsingNumber(header_str.to_string()))
        });
    let length = match r_length {
        Err(ClientError::NoHeader) => {
            eprintln!("Can't find header Content-Length");
            0
        }
        Err(ClientError::ParsingHeaderError(header)) => {
            eprintln!("Can't parse header value: {:?}", header);
            0
        }
        Err(ClientError::ParsingNumber(value)) => {
            eprintln!("Can't parse header value into number: {}", value);
            0
        }
        Ok(length) => length,
    };
    let bar = ProgressBar::new(!0);
    bar.set_message("Downloading");
    bar.set_style(
        ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
            .unwrap()
            .progress_chars("##-"),
    );
    bar.set_length(length);

    let mut file = File::create(file_path)?;
    std::io::copy(&mut response.into_body().into_reader(), &mut file)?;
    Ok(())
}
