use core::ffi::*;

// Base
pub type BOOL = i32;
pub type DWORD = u32;
pub type INT = i32;

// Pointers
pub type PDWORD = *mut DWORD;
pub type LPVOID = *mut c_void;
pub type PWSTR = *const u16;
pub type HWND = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type LPCWSTR = *const u16;

// Token
pub type TOKEN_ACCESS_MASK = u32;
pub type TOKEN_INFORMATION_CLASS = u32;

// Handles
pub type HANDLE = *mut c_void;

pub type HMODULE = *mut c_void;