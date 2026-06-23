use crate::types::*;

#[cfg(feature = "windows_raw_dylib")]
macro_rules! link_raw_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[cfg_attr(not(target_arch = "x86"), link(name = $library, kind = "raw-dylib", modifiers = "+verbatim"))]
        #[cfg_attr(target_arch = "x86", link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated"))]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[cfg(not(feature = "windows_raw_dylib"))]
macro_rules! link_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        // Note: the windows-targets crate uses a pre-built Windows.lib import library which we don't
        // have in this repo. So instead we always link kernel32.lib and add the rest of the import
        // libraries below by using an empty extern block. This works because extern blocks are not
        // connected to the library given in the #[link] attribute.
        #[link(name = "kernel32")]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[cfg(feature = "windows_raw_dylib")]
macro_rules! link {
    ($($tt:tt)*) => (
        link_raw_dylib!($($tt)*);
    )
}

#[cfg(not(feature = "windows_raw_dylib"))]
macro_rules! link {
    ($($tt:tt)*) => (
        link_dylib!($($tt)*);
    )
}

link!("kernel32.dll" "system" fn CloseHandle(hObject: HANDLE) -> BOOL);
link!("kernel32.dll" "system" fn GetCurrentProcess() -> HANDLE);
link!("advapi32.dll" "system" fn OpenProcessToken(processHandle: HANDLE, desiredAccess: TOKEN_ACCESS_MASK, tokenHandle: *mut HANDLE) -> BOOL);
link!("advapi32.dll" "system" fn GetTokenInformation(tokenHandle: HANDLE, tokenInformationClass: TOKEN_INFORMATION_CLASS, tokenInformation: LPVOID, tokenInformationLength: DWORD, returnLength: PDWORD) -> BOOL);
link!("kernel32.dll" "system" fn GetModuleFileNameW(hModule: HMODULE, lpFileName: PWSTR, nSize: u32) -> u32);
link!("shell32.dll" "system" fn ShellExecuteW(hwnd: HWND, lpOperation: LPCWSTR, lpFile: LPCWSTR, lpParameters: LPCWSTR, lpDirectory: LPCWSTR, nShowCmd: INT) -> HINSTANCE);
link!("kernel32.dll" "system" fn ExitProcess(uExitCode: u32) -> !);