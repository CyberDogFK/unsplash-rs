// src/main.rs
// Bring in Cocoa and objc crates.
extern crate cocoa;
extern crate objc;

use std::ffi::OsString;
use cocoa::appkit::{
    NSApplication,
    NSStatusBar,
    NSRunningApplication,
    NSApplicationActivateIgnoringOtherApps,
    NSVariableStatusItemLength,
};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSRect, NSSize, NSPoint, NSString};

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::runtime::{Object, Sel};

//
// GLOBAL_POPOVER: Global storage for our NSPopover instance.
//
static mut GLOBAL_POPOVER: id = nil;

//
// toggle_popover: Called by our toggle delegate to show or hide the popover.
// 'sender' is expected to be the NSStatusItem's button.
//
#[unsafe(no_mangle)]
extern "C" fn toggle_popover(_this: &Object, _cmd: Sel, sender: id) -> () {
    unsafe {
        if GLOBAL_POPOVER != nil {
            let is_shown: i32 = msg_send![GLOBAL_POPOVER, isShown];
            if is_shown != 0 {
                let _: () = msg_send![GLOBAL_POPOVER, close];
            } else {
                let bounds: NSRect = msg_send![sender, bounds];
                let _: () = msg_send![GLOBAL_POPOVER, showRelativeToRect: bounds ofView: sender preferredEdge: 1];
            }
        }
    }
}

//
// create_toggle_delegate: Creates a custom Objective-C delegate that implements toggle_popover:.
//
fn create_toggle_delegate() -> id {
    use objc::declare::ClassDecl;
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("ToggleDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(togglePopover:),
            toggle_popover as extern "C" fn(&Object, Sel, id) -> (),
        );
    }
    decl.register();
    unsafe { msg_send![class!(ToggleDelegate), new] }
}

//
// show_alert: Displays an NSAlert with a title and message.
// This is called when the Alert button is pressed.
//
extern "C" fn show_alert(_this: &Object, _cmd: Sel, _sender: id) -> () {
    unsafe {
        let alert: id = msg_send![class!(NSAlert), new];
        let title = NSString::alloc(nil).init_str("Test Alert");
        let message = NSString::alloc(nil).init_str("This is a test alert.");
        let _: () = msg_send![alert, setMessageText: title];
        let _: () = msg_send![alert, setInformativeText: message];
        let _: () = msg_send![alert, addButtonWithTitle: NSString::alloc(nil).init_str("OK")];
        let _: i32 = msg_send![alert, runModal];
    }
}


//
// create_alert_delegate: Creates a delegate object that implements show_alert:.
//
fn create_alert_delegate() -> id {
    use objc::declare::ClassDecl;
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("AlertDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(showAlert:),
            show_alert as extern "C" fn(&Object, Sel, id) -> (),
        );
    }
    decl.register();
    unsafe { msg_send![class!(AlertDelegate), new] }
}

extern "C" fn update_wallpaper(_this: &Object, _cmd: Sel, _sender: id) -> () {
    unsafe {
        crate::update_image()
    }
}

fn create_refresh_wallpaper_delegate() -> id {
    use objc::declare::ClassDecl;
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("WallpaperUpdateDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(updateWallpaper:),
            update_wallpaper as extern "C" fn(&Object, Sel, id) -> (),
        );
    }
    decl.register();
    unsafe { msg_send![class!(WallpaperUpdateDelegate), new] }
}

//
// create_popover_content_view: Constructs the popover's content view.
// The view includes:
//   - A blurred background (using NSVisualEffectView)
//   - A header with the text "Security Status"
//   - A scrollable list of dummy rows (each row shows a system icon and label)
//   - A footer with a logo, version label, an "Alert" button, and a "Refresh" button
//
// All colors and fonts use system defaults so that dark/light mode is supported automatically.
fn create_popover_content_view(file_name: &str) -> id {
    unsafe {
        // Main content view.
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(420.0, 400.0));
        let content_view: id = msg_send![class!(NSView), alloc];
        let content_view: id = msg_send![content_view, initWithFrame: frame];

        // --- Background: NSVisualEffectView for blur ---
        let effect_view: id = msg_send![class!(NSVisualEffectView), alloc];
        let effect_view: id = msg_send![effect_view, initWithFrame: frame];
        // Use material 6 ("popover" style) – works well in light mode.
        let _: () = msg_send![effect_view, setMaterial: 6];
        let _: () = msg_send![effect_view, setBlendingMode: 0];
        let _: () = msg_send![effect_view, setState: 1];
        let _: () = msg_send![content_view, addSubview: effect_view];

        // --- Header ---
        let header_frame = NSRect::new(NSPoint::new(12.0, 360.0), NSSize::new(396.0, 30.0));
        let header: id = msg_send![class!(NSTextField), alloc];
        let header: id = msg_send![header, initWithFrame: header_frame];
        let header_str = NSString::alloc(nil).init_str("Security Status");
        let _: () = msg_send![header, setStringValue: header_str];
        // Bold system font (16pt).
        let header_font: id = msg_send![class!(NSFont), boldSystemFontOfSize: 16.0];
        let _: () = msg_send![header, setFont: header_font];
        // Use dynamic text color.
        let label_color: id = msg_send![class!(NSColor), labelColor];
        let _: () = msg_send![header, setTextColor: label_color];
        let _: () = msg_send![header, setBezeled: NO];
        let _: () = msg_send![header, setDrawsBackground: NO];
        let _: () = msg_send![header, setEditable: NO];
        let _: () = msg_send![header, setSelectable: NO];
        let _: () = msg_send![content_view, addSubview: header];

        // --- Divider ---
        let divider_frame = NSRect::new(NSPoint::new(12.0, 350.0), NSSize::new(396.0, 1.0));
        let divider: id = msg_send![class!(NSBox), alloc];
        let divider: id = msg_send![divider, initWithFrame: divider_frame];
        let _: () = msg_send![divider, setBoxType: 1]; // Separator style.
        let _: () = msg_send![content_view, addSubview: divider];

        // --- Menu Items List (Scroll View) ---
        let scroll_frame = NSRect::new(NSPoint::new(12.0, 100.0), NSSize::new(396.0, 240.0));
        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame: scroll_frame];
        let doc_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(396.0, 240.0));
        let doc_view: id = msg_send![class!(NSView), alloc];
        let doc_view: id = msg_send![doc_view, initWithFrame: doc_frame];

        let _: () = msg_send![scroll_view, setDocumentView: doc_view];
        let _: () = msg_send![content_view, addSubview: scroll_view];

        // Wallpaper presentation: NSImageView using downloaded image
        // let logo_frame = NSRect::new(NSPoint::new(10.0, 20.0), NSSize::new(140.0, 140.0));
        let logo: id = msg_send![class!(NSImageView), alloc];
        let logo: id = msg_send![logo, initWithFrame: frame];
        let logo_str = NSString::alloc(nil).init_str(&format!("wallpaper/{}", file_name));
        let alloc: id = msg_send![class!(NSImage), alloc];
        let logo_img: id = msg_send![alloc, initWithContentsOfFile: logo_str];
        let _: () = msg_send![logo, setImage: logo_img];
        // let _: () = msg_send![logo_img, setTemplate: YES];
        let _: () = msg_send![doc_view, addSubview: logo];

        // --- Footer ---
        let footer_frame = NSRect::new(NSPoint::new(12.0, 12.0), NSSize::new(396.0, 60.0));
        let footer: id = msg_send![class!(NSView), alloc];
        let footer: id = msg_send![footer, initWithFrame: footer_frame];

        // Version label.
        let version_frame = NSRect::new(NSPoint::new(60.0, 30.0), NSSize::new(100.0, 20.0));
        let version: id = msg_send![class!(NSTextField), alloc];
        let version: id = msg_send![version, initWithFrame: version_frame];
        let version_str = NSString::alloc(nil).init_str("v0.0.1");
        let _: () = msg_send![version, setStringValue: version_str];
        let version_font: id = msg_send![class!(NSFont), systemFontOfSize: 12.0];
        let _: () = msg_send![version, setFont: version_font];
        let _: () = msg_send![version, setTextColor: label_color];
        let _: () = msg_send![version, setBezeled: NO];
        let _: () = msg_send![version, setDrawsBackground: NO];
        let _: () = msg_send![version, setEditable: NO];
        let _: () = msg_send![version, setSelectable: NO];
        let _: () = msg_send![footer, addSubview: version];

        // Alert button.
        let alert_frame = NSRect::new(NSPoint::new(260.0, 20.0), NSSize::new(60.0, 30.0));
        let alert_btn: id = msg_send![class!(NSButton), alloc];
        let alert_btn: id = msg_send![alert_btn, initWithFrame: alert_frame];
        let alert_title = NSString::alloc(nil).init_str("Alert");
        let _: () = msg_send![alert_btn, setTitle: alert_title];
        let alert_delegate = create_alert_delegate();
        let _: () = msg_send![alert_btn, setTarget: alert_delegate];
        let _: () = msg_send![alert_btn, setAction: sel!(showAlert:)];
        let _: () = msg_send![footer, addSubview: alert_btn];

        // Refresh button.
        let refresh_frame = NSRect::new(NSPoint::new(320.0, 20.0), NSSize::new(60.0, 30.0));
        let refresh: id = msg_send![class!(NSButton), alloc];
        let refresh: id = msg_send![refresh, initWithFrame: refresh_frame];
        let refresh_title = NSString::alloc(nil).init_str("Refresh");
        let _: () = msg_send![refresh, setTitle: refresh_title];
        let wallpaper_delegate = create_refresh_wallpaper_delegate();
        let _: () = msg_send![refresh, setTarget: wallpaper_delegate];
        let _: () = msg_send![refresh, setAction: sel!(updateWallpaper:)];
        let _: () = msg_send![footer, addSubview: refresh];

        let _: () = msg_send![content_view, addSubview: footer];

        content_view
    }
}

//
// create_popover_view_controller: Wraps our content view in an NSViewController.
//
fn create_popover_view_controller(file_name: &str) -> id {
    unsafe {
        let view = create_popover_content_view(file_name);
        let vc: id = msg_send![class!(NSViewController), alloc];
        let vc: id = msg_send![vc, init];
        let _: () = msg_send![vc, setView: view];
        vc
    }
}

//
// setup_status_item_and_popover: Creates the tray icon and associates a popover with it.
//
pub fn setup_status_item_and_popover(file_name: &str) {
    unsafe {
        let _app = NSApplication::sharedApplication(nil);
        let status_bar: id = NSStatusBar::systemStatusBar(nil);
        let status_item: id = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

        set_tray_icon(status_item);

        create_popover(file_name);

        // Create a toggle delegate.
        let toggle_delegate: id = create_toggle_delegate();

        // Set the status item button's target and action.
        let button: id = msg_send![status_item, button];
        if button != nil {
            let _: () = msg_send![button, setTarget: toggle_delegate];
            let _: () = msg_send![button, setAction: sel!(togglePopover:)];
        }

        NSRunningApplication::currentApplication(nil)
            .activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }
}

pub fn create_popover(file_name: &str) {
    unsafe {
        // Create an NSPopover.
        let popover: id = msg_send![class!(NSPopover), alloc];
        let popover: id = msg_send![popover, init];
        let vc = create_popover_view_controller(file_name);
        let _: () = msg_send![popover, setContentViewController: vc];

        GLOBAL_POPOVER = popover;
    }
}

fn set_tray_icon(status_item: id) {
    unsafe {
        // Set the tray icon using "rainbow".
        let symbol_name = NSString::alloc(nil).init_str("rainbow");
        let image: id = msg_send![class!(NSImage), imageWithSystemSymbolName: symbol_name accessibilityDescription: nil];
        let _: () = msg_send![status_item, setImage: image];
        let _: () = msg_send![image, setTemplate: YES];

    }
}

// Dummy rows with varied system icons.
// let icons = ["doc.text", "exclamationmark.circle", "checkmark.circle", "questionmark"];
// for i in 0..4 {
//     let row_y = 240.0 - ((i + 1) as f64 * 50.0);
//     let row_frame = NSRect::new(NSPoint::new(0.0, row_y), NSSize::new(396.0, 40.0));
//     let row: id = msg_send![class!(NSView), alloc];
//     let row: id = msg_send![row, initWithFrame: row_frame];
//
//     // Icon.
//     let icon_frame = NSRect::new(NSPoint::new(10.0, 10.0), NSSize::new(20.0, 20.0));
//     let icon_view: id = msg_send![class!(NSImageView), alloc];
//     let icon_view: id = msg_send![icon_view, initWithFrame: icon_frame];
//     let icon_str = NSString::alloc(nil).init_str(icons[i % icons.len()]);
//     let icon_img: id = msg_send![class!(NSImage), imageWithSystemSymbolName: icon_str accessibilityDescription: nil];
//     let _: () = msg_send![icon_view, setImage: icon_img];
//     let _: () = msg_send![icon_img, setTemplate: YES];
//     let _: () = msg_send![row, addSubview: icon_view];
//
//     // Label.
//     let label_frame = NSRect::new(NSPoint::new(35.0, 10.0), NSSize::new(250.0, 20.0));
//     let label: id = msg_send![class!(NSTextField), alloc];
//     let label: id = msg_send![label, initWithFrame: label_frame];
//     let text = NSString::alloc(nil).init_str(&format!("Menu Item {}", i + 1));
//     let _: () = msg_send![label, setStringValue: text];
//     let label_font: id = msg_send![class!(NSFont), systemFontOfSize: 14.0];
//     let _: () = msg_send![label, setFont: label_font];
//     let _: () = msg_send![label, setTextColor: label_color];
//     let _: () = msg_send![label, setBezeled: NO];
//     let _: () = msg_send![label, setDrawsBackground: NO];
//     let _: () = msg_send![label, setEditable: NO];
//     let _: () = msg_send![label, setSelectable: NO];
//     let _: () = msg_send![row, addSubview: label];
//
//     // Circular indicator.
//     let circle_frame = NSRect::new(NSPoint::new(320.0, 10.0), NSSize::new(20.0, 20.0));
//     let circle: id = msg_send![class!(NSView), alloc];
//     let circle: id = msg_send![circle, initWithFrame: circle_frame];
//     let _: () = msg_send![circle, setWantsLayer: YES];
//     let layer: id = msg_send![circle, layer];
//     let indicator_color: id = msg_send![class!(NSColor), systemGreenColor];
//     let _: () = msg_send![layer, setBackgroundColor: indicator_color];
//     let _: () = msg_send![layer, setCornerRadius: 10.0];
//     let _: () = msg_send![row, addSubview: circle];
//
//     let _: () = msg_send![doc_view, addSubview: row];
// }
