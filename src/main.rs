use windows_sys::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    let mut window_pids: Vec<(String, isize)> = Vec::new();
    unsafe {
        EnumWindows(Some(enum_window), &mut window_pids as *mut Vec<(String,isize)> as LPARAM);        
        
    }

    for pid in window_pids.iter() {
        println!("{:?}", pid);
    }

    
}


extern "system" fn enum_window(window: HWND, window_pids: LPARAM) -> BOOL {
    unsafe {
        let window_pids = window_pids as *mut Vec<(String, isize)>;
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);
        let mut pid = 0;
        GetWindowThreadProcessId(window, &mut pid);

        if !text.is_empty() && IsWindowVisible(window) != 0{
            (*window_pids).push((text, pid as isize));
        }

        1
    }
}
