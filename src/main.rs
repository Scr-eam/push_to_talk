/*
    made because discord plays some dumb audio whenever you use push to talk,
    and because roblox doesn't support push to talk, this will work universally

    //TODO: implement capability to add different keys for different windows
*/

use winapi::um::winuser::{GetKeyState, VK_XBUTTON2};
use std::{time, thread};

use windows::Win32::{
    Foundation::{HWND, LPARAM},
    System::SystemServices::APPCOMMAND_MICROPHONE_VOLUME_MUTE,
    UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, SendMessageW, WM_APPCOMMAND,
    }
};

static mut MUTED: bool = false; // this should be your default microphone state, set it to true or false
static mut KEY: i32 = VK_XBUTTON2; // change this to any key u want, just make sure to include it

fn main() {

    let delay = time::Duration::from_millis(1);

    let active_window_hwnd = get_foreground_window();

    // disable mic if not already disabled
    if !unsafe { MUTED } {
        toggle_microphone(active_window_hwnd);
    }

    loop {
        let active_window_hwnd = get_foreground_window();
        
        //let window_title = get_window_title(active_window_hwnd).expect("failed to get window title");
    
        let key_state = unsafe { GetKeyState(KEY) } as u16;
    
        let is_key_down = (key_state & 0x8000) != 0;

        let is_muted = unsafe { MUTED };

        // i couldn't figure out how to check if the mic was muted so i made my own boolean

        if is_key_down && is_muted {
            toggle_microphone(active_window_hwnd);
        } else if !is_muted && !is_key_down {
            toggle_microphone(active_window_hwnd);
        }

        thread::sleep(delay);
    }
}

fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

fn toggle_microphone(hwnd: HWND) {
    unsafe {
        SendMessageW(hwnd, WM_APPCOMMAND, None, LPARAM((APPCOMMAND_MICROPHONE_VOLUME_MUTE.0 << 16) as isize));
        MUTED = !MUTED;
    };
}

// this function will be used to window checking
fn _get_window_title(hwnd: HWND) -> Result<String, ()> {
    let window_title: String;

    unsafe {
        let mut wnd_title: Vec<u16> = vec![0; 255];
        let length = GetWindowTextW(hwnd, &mut wnd_title);
        
        window_title = String::from_utf16_lossy(&wnd_title[0..(length as usize)]);
    }

    Ok(window_title)
}