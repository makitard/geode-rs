use crate::mod_::Mod;

pub struct Loader(pub(crate) *const ());

impl Loader {
    pub fn get() -> Self {
        unsafe {
            Self(geo_fn!(
                unsafe extern "C" fn() -> *const (),
                "?get@Loader@geode@@SAPAV12@XZ"
            )())
        }
    }

    pub fn take_next_mod(&self) -> Mod {
        unsafe {
            Mod(geo_fn!(
                unsafe extern "thiscall" fn(*const ()) -> *const (),
                "?takeNextMod@Loader@geode@@IAEPAVMod@2@XZ"
            )(self.0))
        }
    }
}
