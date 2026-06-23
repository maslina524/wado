use core::ffi::*;
use crate::types::*;

// Windows consts
pub const MAX_PATH: usize = 260;

pub const NULL: *mut c_void = 0 as *mut c_void;

pub const TOKEN_QUERY: u32 = 0x0008;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: DWORD
}

pub const TokenElevation: u32 = 20;

pub const SW_SHOWNORMAL: INT = 1;