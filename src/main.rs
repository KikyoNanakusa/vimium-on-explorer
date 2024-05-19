use windows::{core::*, Win32::{Foundation::{LPARAM, LRESULT, WPARAM, *}, 
System::Threading::*, 
UI::{Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_DOWN, VK_H, VK_J, VK_K, VK_L, VK_LEFT, VK_RIGHT, VK_UP}, 
WindowsAndMessaging::{CallNextHookEx, SetWindowsHookExA, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, *}}},
};

use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};
use std::{thread, time};

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref TARGET_HWND: Mutex<HWND> = Mutex::new(HWND::default());
}

fn main() {
    unsafe {
        MessageBoxW(None, w!("Hello, Windows!"), w!("Dialog Box Example"), MB_OK);
        
        let app_name = w!("C:\\Windows\\explorer.exe");
        let mut startup_info = STARTUPINFOW::default(); 
        let mut process_information = PROCESS_INFORMATION::default();

        CreateProcessW(
            app_name,
            PWSTR::null(),
            Some(null_mut()),
            Some(null_mut()),
            BOOL::from(false),
            CREATE_NEW_CONSOLE,
            Some(null_mut()),
            PCWSTR::null(),
            &mut startup_info,
            &mut process_information,
        ).expect("Failed to create process");

        println!("Process ID: {}", process_information.dwProcessId);
        thread::sleep(time::Duration::from_secs(3));
        let mut target_hwnd = TARGET_HWND.lock().unwrap();
        *target_hwnd = find_explorer_window().expect("Failed to find explorer window.");
        set_window_title(*target_hwnd, "Vimsplorer").expect("Failed to set window title.");


        let k_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(k_callback1), HINSTANCE::default(), 0);
        let mut message = MSG::default();
        while GetMessageA(&mut message, *target_hwnd, 0, 0).into() {
            DispatchMessageA(&message);
        }

        UnhookWindowsHookEx(k_hook.unwrap()).expect("Failed to unhook.");
    }
}

fn find_explorer_window() -> std::result::Result<HWND, String> {
    let input_string = "CabinetWClass";
    let encoded_wide_string: Vec<u16> = OsStr::new(input_string)
        .encode_wide() // UTF-16にエンコード
        .chain(std::iter::once(0)) // Null終端を追加
        .collect();
    unsafe {
        let hwnd = FindWindowW(PCWSTR(encoded_wide_string.as_ptr()), PCWSTR::null());
        if hwnd.0 == 0 {
            println!("No window found.");
            Err(String::from("No window found."))
        } else {
            println!("Found window handle: {:?}", hwnd);
            Ok(hwnd)
        }
    }
}

extern "system" fn k_callback1(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if wparam.0 as u32 == WM_KEYDOWN {
            let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let vk_code_inner = &*(lparam.0 as *const u16) as &u16;
            dbg!(vk_code_inner);

            let new_vk: VIRTUAL_KEY = match kb_struct.vkCode {
                37 => VK_H, // left
                40 => VK_J, // down
                39 =>VK_L,  // right
                38 => VK_K, // up
                _ => VIRTUAL_KEY(0),
            };
            dbg!(new_vk);
            dbg!("------------------------");

            if new_vk != VIRTUAL_KEY(0) {
                let mut kbd = KEYBDINPUT::default();
                kbd.wVk = new_vk;
                kbd.dwFlags = KEYBD_EVENT_FLAGS::default(); // KEYDOWN
                kbd.time = 0; 
                kbd.dwExtraInfo = GetMessageExtraInfo().0 as usize;
                let mut input = INPUT::default();
                input.r#type = INPUT_KEYBOARD;
                input.Anonymous.ki = kbd;
                let result = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
                dbg!(result);
            }
        }

        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}

fn message_loop(hwnd: HWND) -> std::result::Result<(), windows::core::Error> {
    unsafe {
        let mut msg = MSG::default();

        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            if msg.hwnd == hwnd {
                // ここで msg.message をチェックし、必要な処理を行います
                println!("Received message: {}", msg.message);
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        Ok(())
    }
}

fn set_window_title(hwnd: HWND, title: &str) -> std::result::Result<(), String> {
    let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        // WM_SETTEXT メッセージを送信してウィンドウのタイトルを更新
        let result = SendMessageW(hwnd, WM_SETTEXT, WPARAM(0), LPARAM(title_wide.as_ptr() as isize));
        if result == LRESULT(0) {
            Err("Failed to set window title.".to_string())
        } else {
            Ok(())
        }
    }
}