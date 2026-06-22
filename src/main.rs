#![no_std]
#![no_main]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]

use core::{ffi::c_void, mem, ptr::addr_of_mut};

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

use consts::*;
use types::*;
use link::*;

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

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i32 {
    return if is_elevated() { 1 } else { 0 }
}