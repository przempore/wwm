use crate::tray_icon::show_tray_icon;
use bindings::windows::{
    win32::{
        display_devices::RECT,
        windows_and_messaging::{GetForegroundWindow, GetWindowRect, GetWindowTextW, HWND},
    },
    TRUE,
};
use mouse_rs::Mouse;

mod tray_icon;

fn main() -> Result<(), systray::Error> {
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
            println!("GetWindowRect failed!",);
            return Ok(());
        }
    }

    println!("{:?}", rect);

    let mouse = Mouse::new();
    mouse
        .move_to(rect.right - 6, rect.bottom - 6)
        .expect("Unable to move mouse");

    show_tray_icon()
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
