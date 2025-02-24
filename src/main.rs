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
    let t =
        r#"tell application "System Events" tell every desktop
        set picture to "imagePath"
        end tell
        end tell"#;
    App::new("com.hello.world", BasicApp::default()).run();
}
