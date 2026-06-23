#![no_std]
#![no_main]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]

use core::{ffi::c_void, mem, ptr::{self, addr_of_mut}};

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod consts;
mod types;
mod link;
mod encoding;

use consts::*;
use types::*;
use link::*;

use encoding::str_to_utf16;

fn is_elevated() -> bool {
    let mut h_token: HANDLE = NULL;
    let current_process = unsafe { GetCurrentProcess() };
    let process_token = unsafe { !OpenProcessToken(current_process, TOKEN_QUERY, &mut h_token) };

    if process_token == 1 {
        return false;
    }

    let mut elevation = TOKEN_ELEVATION::default();
    let mut size: DWORD = mem::size_of::<TOKEN_ELEVATION>() as DWORD;

    let success = unsafe {
        GetTokenInformation(
            h_token, 
            TokenElevation, 
            addr_of_mut!(elevation).cast::<c_void>(), 
            size, 
            &mut size
        )
    };
    unsafe { CloseHandle(h_token) };

    return success == 1 && elevation.TokenIsElevated == 1;
}

fn run_as_admin() -> bool {
    let mut exe_path = [0u16; MAX_PATH];

    let len = unsafe {
        GetModuleFileNameW(ptr::null_mut(), exe_path.as_mut_ptr(), MAX_PATH as u32)
    };
    if len == 0 {
        return false;
    }

    let mut runas = [0u16; 16];
    let len = str_to_utf16("runas", &mut runas);

    let result = unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            runas[..len].as_ptr(),
            exe_path.as_ptr(),
            ptr::null(),
            ptr::null(),
            SW_SHOWNORMAL as i32,
        )
    };

    result as isize > 32
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    let exit_code = if !is_elevated() {
        if run_as_admin() { 0 } else { 1 }
    } else {
        8
    };
    unsafe { ExitProcess(exit_code as u32) };
}