use windows_sys::{
    Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*, 
    Win32::System::Threading::*,
    Win32::System::Memory::*,
    Win32::System::Diagnostics::Debug::*,
};
use std::ffi::c_void;
use std::mem::MaybeUninit;

fn main() {
    let access_rights = PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION | PROCESS_READ_CONTROL | PROCESS_VM_OPERATION;
    let mut window_pids: Vec<(String, u32)> = Vec::new();

    unsafe {
        EnumWindows(Some(enum_window), &mut window_pids as *mut Vec<(String, u32)> as LPARAM);        
    }

    let read_pid = window_pids.iter().find(|p| p.0 == "Command Prompt").unwrap().1;
        
    let process = unsafe { OpenProcess(access_rights, FALSE, read_pid) } as HANDLE;
    if process == std::ptr::null_mut(){
        unsafe {println!("ERROR: {}", GetLastError()) };
    }

    let mut address: usize = 0;
    let mut buf: [u8;4096] = [0; 4096];
    let mut num_read: usize = 0;

    loop {
        unsafe {
            let base_address = address as *const c_void;
            let mut mbi = MaybeUninit::<MEMORY_BASIC_INFORMATION>::uninit();
            
            let result = VirtualQueryEx(
            process,
            base_address,
            mbi.as_mut_ptr(),
            size_of::<MEMORY_BASIC_INFORMATION>());

            if result == 0 { println!("{}", GetLastError()); break;}
            let mbi = mbi.assume_init();

            if mbi.State == MEM_COMMIT &&
                mbi.Protect == PAGE_READWRITE &&
                mbi.Protect != PAGE_GUARD {
    
                let result = ReadProcessMemory(
                    process,
                    base_address,
                    &mut buf as *mut _ as *mut c_void,
                    4096,
                    &mut num_read
                );
                
                println!("here3");
                println!("{}", num_read);
                if result == 0 { println!("{}", GetLastError()); break;}

                print_hex(&buf, address); 

            }

            address = mbi.BaseAddress as usize + mbi.RegionSize;
        }
    }
}


extern "system" fn enum_window(window: HWND, window_pids: LPARAM) -> BOOL {
    unsafe {
        let window_pids_ptr = window_pids as *mut Vec<(String, u32)>;
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);
        let mut pid = 0;
        GetWindowThreadProcessId(window, &mut pid);

        if !text.is_empty() && IsWindowVisible(window) != 0{
            (*window_pids_ptr).push((text, pid as u32));
        }

        1
    }
}

fn print_hex(buffer: &[u8], base_address: usize) {
    for (i, chunk) in buffer.chunks(16).enumerate() {
        let current_address = base_address + i * 16;
        
        print!("{:016X}: ", current_address);
        
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        
        if chunk.len() < 16 {
            for _ in 0..(16 - chunk.len()) {
                print!("   ");
            }
        }
        
        print!(" |");
        
        for byte in chunk {
            if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        
        println!("|");
    }
}

