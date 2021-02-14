//importing in execute! macro
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use crate::tray_icon::show_tray_icon;
use bindings::windows::{
    win32::{
        display_devices::RECT,
        windows_and_messaging::{GetForegroundWindow, GetWindowRect, GetWindowTextW, HWND},
    },
    TRUE,
};
use mouse_rs::{types::keys::Keys, Mouse};

mod tray_icon;

fn main() {
    let please_stop = Arc::new(AtomicBool::new(false));

    // let keyboard_events_thread = thread::spawn(listen_for_key_events);
    let keyboard_events_thread = thread::spawn({
        let should_i_stop = please_stop.clone();
        move || listen_for_key_events(should_i_stop.clone())
    });
    if let Err(e) = show_tray_icon() {
        println!("{:?}", e);
        return;
    }

    please_stop.store(true, Ordering::SeqCst);

    match keyboard_events_thread.join() {
        Ok(_ok) => println!("All good"),
        Err(e) => println!("{:?}", e),
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

fn listen_for_key_events(should_i_stop: Arc<AtomicBool>) {
    let mut stdout = stdout();
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(r#"ctrl + q to exit, ctrl + h to print "Hello world", alt + t to print "crossterm is cool""#))
            .unwrap();

    //key detection
    loop {
        if should_i_stop.load(Ordering::SeqCst) {
            break;
        }
        //going to top left corner
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

        //matching the key
        match read().unwrap() {
            //i think this speaks for itself
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                //clearing the screen and printing our message
                // }) => execute!(stdout, Clear(ClearType::All), Print("Hello world!")).unwrap(),
            }) => match grab_and_resize() {
                Ok(_ok) => println!("All good"),
                Err(e) => println!("{:?}", e),
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('t'),
                modifiers: KeyModifiers::ALT,
            }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            _ => (),
        }
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}
