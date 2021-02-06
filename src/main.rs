use bindings::windows::win32::windows_and_messaging::{GetForegroundWindow, GetWindowTextW};

fn main() {
    match get_foreground_window_title() {
        Err(_e) => println!(""),
        Ok(title) => println!("Title: {}", title),
    };
}

fn get_foreground_window_title() -> Result<String, ()> {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let window = GetForegroundWindow();
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            return Ok(text);
        }
    }

    Err(())
}
