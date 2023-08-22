//! Rust bindings for the [Geode SDK](https://geode-sdk.org)
// //!
// //! ## Usage
// //!

#![feature(abi_thiscall, stmt_expr_attributes)]

//doing #[link(name = "Geode", kind = "raw-dylib")] didn't work :(
#[doc(hidden)]
macro_rules! geo_fn {
    ($target_ty:ty, $sym_name:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            let base = windows::Win32::System::LibraryLoader::GetModuleHandleA(windows::core::s!(
                "Geode.dll"
            ))
            .unwrap();

            let c_name = std::ffi::CString::new($sym_name).unwrap();

            let address = windows::Win32::System::LibraryLoader::GetProcAddress(
                base,
                windows::core::PCSTR(c_name.as_ptr() as _),
            )
            .unwrap() as usize;
            std::mem::transmute::<_, $target_ty>(address)
        }
    };
}

///Defines the entry point for the mod.
///Requires the `windows` crate with the following features: ["Win32_Foundation", "Win32_System_Threading", "Win32_Security", "Win32_System_LibraryLoader"].
#[macro_export]
macro_rules! entry {
    ($($exprs:tt)*) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        unsafe extern "system" fn DllMain(
            _module: *const (),
            call_reason: u32,
            _reserved: *const (),
        ) -> u32 {
            if call_reason == 1 {
                windows::Win32::System::Threading::CreateThread(
                    None,
                    0,
                    Some(main_thread),
                    Some(_module as _),
                    windows::Win32::System::Threading::THREAD_CREATION_FLAGS(0),
                    None,
                )
                .unwrap();
            }

            1
        }

        unsafe extern "system" fn main_thread(_: *mut core::ffi::c_void) -> u32 {
            $($exprs)*
            0
        }
    }
}

pub mod hook;
pub mod loader;
pub mod log;
pub mod mod_;
pub mod prelude;
pub mod stl;
pub mod tulip;
pub mod utils;
//pub mod event; not done yet lol
