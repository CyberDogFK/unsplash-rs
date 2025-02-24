use std::process::Command;
use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello world!");
        self.window.show();
    }
}

fn main() {
    let file_path = "/Users/antonpavliuk/projects/learning/rust/macos/unsplash-rs/the-chaffins-syhIpeHdLdM-unsplash.jpg";
    let cmd = format!(r#"tell application "System Events"
tell every desktop
set picture to "{}"
end tell
end tell
"#,
    file_path);
    println!("{}", &cmd);
    Command::new("osascript").args(&["-e", &cmd]).output().unwrap();
}
