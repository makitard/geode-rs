///Returns the base address of Geometry Dash
pub unsafe fn base() -> usize {
    windows::Win32::System::LibraryLoader::GetModuleHandleA(windows::core::PCSTR(0 as _))
        .unwrap()
        .0 as usize
}
