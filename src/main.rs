use crate::tray_icon::show_tray_icon;
use bindings::windows::{
    win32::{
        display_devices::RECT,
        windows_and_messaging::{GetForegroundWindow, GetWindowRect, GetWindowTextW, HWND},
    },
    TRUE,
};
use mouse_rs::{types::keys::Keys, Mouse};
use std::thread;

mod tray_icon;

fn main() {
    let mouse_events_thread = thread::spawn(grab_and_resize);
    match mouse_events_thread.join() {
        Ok(_ok) => println!("All good"),
        Err(e) => println!("{:?}", e),
    }

    if let Err(e) = show_tray_icon() {
        println!("{:?}", e);
        return;
    }
}

fn grab_and_resize() -> Result<(), String> {
    let window: HWND;
    unsafe {
        window = GetForegroundWindow();
    }

    match get_window_title(window) {
        Err(e) => println!("{:?}", e),
        Ok(title) => println!("Title: {}", title),
    };

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe {
        let success = GetWindowRect(window, &mut rect);
        if success != TRUE {
            return Err("GetWindowRect failed!".to_string());
        }
    }

    println!("{:?}", rect);

    let mouse = Mouse::new();
    mouse
        .move_to(rect.right - 9, rect.bottom - 8)
        .expect("Unable to move mouse");

    mouse.press(&Keys::LEFT).expect("Unable to press button");

    Ok(())
}

fn get_window_title(window: HWND) -> Result<String, ()> {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let title = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            return Ok(title);
        }
    }
    Err(())
}
