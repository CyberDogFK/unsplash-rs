mod unsplash;

use std::env;
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
        .args(&["-e", &cmd])
        .output()
        .unwrap();
}

fn main() {
    dotenv::dotenv().ok();
    let access_key = env::var("API_ACCESS_KEY")
        .expect("Can't find environment variable API_ACCESS_KEY");
    let random_image: ImageBody = unsplash::get_random_image(&access_key).unwrap();
    let file_path = "new_file.jpg";
    unsplash::download_image(&random_image.urls.full, file_path, &access_key).unwrap();
    change_wallpaper(file_path);
}
