use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::str::FromStr;
use indicatif::{ProgressBar, ProgressStyle};

const DOWNLOAD_TEMPLATE: &str =
    "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

pub fn get_random_image(access_key: &str) -> Result<String, Box<dyn Error>> {
    let response = ureq::get("https://api.unsplash.com/photos/random")
        .query_pairs(vec![("orientation", "landscape")])
        .header("Authorization", format!("Cleint-ID {}", access_key))
}

pub fn download_image<P: AsRef<Path>>(file_path: P, access_key: &str) -> Result<(), Box<dyn Error>> {
    let response = ureq::get("https://images.unsplash.com/photo-1739609579483-00b49437cc45?crop=entropy&cs=srgb&fm=jpg&ixid=M3w3MTQ0MjJ8MHwxfGFsbHwxfHx8fHx8fHwxNzQwNDg0OTA0fA&ixlib=rb-4.0.3&q=85")
        .header("Authorization", format!("Client-ID {}", access_key))
        .call()?;

    let length = response.headers().get("Content-Length").unwrap();
    let l: u64 = u64::from_str(length.to_str().unwrap()).unwrap();
    println!("{}", l);

    let bar = ProgressBar::new(!0);
    bar.set_message("Downloading");
    bar.set_style(
        ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
            .unwrap()
            .progress_chars("##-"),
    );
    bar.set_length(l);
    
    let mut file = File::create(file_path)?;
    std::io::copy(&mut response.into_body().into_reader(), &mut file)?;
    Ok(())
}
