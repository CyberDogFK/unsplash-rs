mod unsplash;

use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use std::process::Command;

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
        r#"tell application "System Events"
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
    let file_name = "the-chaffins-syhIpeHdLdM-unsplash.jpg";
    let path = "new_file.jpg";
    unsplash::test_get_image(path).unwrap();
    change_wallpaper(path);
}
