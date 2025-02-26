mod unsplash;

use std::{env, fs};
use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use std::process::Command;
use crate::unsplash::ImageBody;

#[derive(Default)]
struct BasicApp {
    window: Window,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello world!");
        self.window.show();
    }
}

fn change_wallpaper(file_name: &str) {
    let file_path = format!(
        "/Users/antonpavliuk/projects/learning/rust/macos/unsplash-rs/{}",
        file_name
    );
    let cmd = format!(
        r#"
tell application "System Events"
tell every desktop
set picture to "{}"
end tell
end tell
"#,
        file_path
    );
    println!("{}", &cmd);
    Command::new("osascript")
        .args(["-e", &cmd])
        .output()
        .unwrap();
}

fn main() {
    dotenv::dotenv().ok();
    let access_key = env::var("API_ACCESS_KEY")
        .expect("Can't find environment variable API_ACCESS_KEY");
    let random_image: ImageBody = unsplash::get_random_image(&access_key).unwrap();
    
    fs::remove_dir_all("wallpaper").unwrap();
    fs::create_dir("wallpaper").unwrap();
    let file_path = format!("./wallpaper/{}.jpg", random_image.slug);
    unsplash::download_image(&random_image.urls.full, &file_path, &access_key).unwrap();
    change_wallpaper(&file_path);
}
