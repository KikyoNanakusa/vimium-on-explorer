use windows::{core::*, Win32::Foundation::*,  Win32::UI::WindowsAndMessaging::*, Win32::System::Threading::*};
use std::ptr::null_mut;

fn main() {
    unsafe {
        MessageBoxW(None, w!("Hello, Windows!"), w!("Dialog Box Example"), MB_OK);
        
        let app_name = w!("C:\\Windows\\explorer.exe");
        let mut startup_info = STARTUPINFOW::default(); 
        let mut process_information = PROCESS_INFORMATION::default();

        let result = CreateProcessW(
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
        );

        match result {
            Ok(_) => {
                println!("Process created successfully!");
            },
            Err(e) => {
                println!("Error creating process: {:?}", e);
            }
        }
    }
}
