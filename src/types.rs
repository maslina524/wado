use core::ffi::*;

// Base
pub type BOOL = i32;
pub type DWORD = u32;

// Pointers
pub type PDWORD = *mut DWORD;
pub type LPVOID = *mut c_void;

// Token
pub type TOKEN_ACCESS_MASK = u32;
pub type TOKEN_INFORMATION_CLASS = u32;

// Handles
pub type HANDLE = *mut c_void;