use windows_sys::{
    Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*, 
    Win32::System::Threading::*,
    Win32::System::Memory::*,
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
    println!("{:?}", process);
    if process == std::ptr::null_mut(){
        unsafe {println!("ERROR: {}", GetLastError()) };
    }

    let mut address: usize = 0;

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
            print_memory_basic_information(&mbi);
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


fn print_memory_basic_information(mbi: &MEMORY_BASIC_INFORMATION) {
    println!("Memory Basic Information:");
    println!("  BaseAddress: {:p}", mbi.BaseAddress);
    println!("  AllocationBase: {:p}", mbi.AllocationBase);
    println!("  AllocationProtect: {}", mbi.AllocationProtect);
    println!("  RegionSize: {}", mbi.RegionSize);
    println!("  State: {}", get_memory_state(mbi.State));
    println!("  Protect: {}", mbi.Protect);
    println!("  Type: {}", mbi.Type);
}

fn get_memory_state(state: u32) -> &'static str {
    match state {
        MEM_COMMIT => "Committed",
        MEM_FREE => "Free",
        MEM_RESERVE => "Reserved",
        _ => "Unknown",
    }
}
