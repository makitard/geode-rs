use crate::hook::Hook;
use crate::stl::{GeodeResult, StlString, StlVector};

pub struct Mod(pub(crate) *const ());

static mut SHARED_MOD: Mod = Mod(0 as _);

impl Mod {
    pub fn get_mut() -> &'static mut Self {
        unsafe {
            if SHARED_MOD.0 as usize == 0 {
                SHARED_MOD = crate::loader::Loader::get().take_next_mod();
            }

            &mut SHARED_MOD
        }
    }

    pub fn get() -> &'static Self {
        &*Self::get_mut()
    }

    pub fn add_hook(&mut self, hook: &Hook) -> Result<(), String> {
        let result = GeodeResult::<*const (), StlString>::empty();
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?addHook@Mod@geode@@QAE?AV?$Result@PAVHook@geode@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0);
            if let Err(err) = result.to_result() {
                Err(err.to_string())
            } else {
                Ok(())
            }
        }
    }

    pub fn enable_hook(&mut self, hook: &Hook) -> Result<(), String> {
        let result = GeodeResult::<(), StlString>::empty();
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?enableHook@Mod@geode@@QAE?AV?$Result@Umonostate@std@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0);
            if let Err(err) = result.to_result() {
                Err(err.to_string())
            } else {
                Ok(())
            }
        }
    }

    pub fn disable_hook(&mut self, hook: &Hook) -> Result<(), String> {
        let result = GeodeResult::<(), StlString>::empty();
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?disableHook@Mod@geode@@QAE?AV?$Result@Umonostate@std@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0);
            if let Err(err) = result.to_result() {
                Err(err.to_string())
            } else {
                Ok(())
            }

        }
    }

    pub fn patch(&self, address: usize, bytes: Vec<u8>) -> Result<Patch, String> {
        let result = GeodeResult::<*const (), StlString>::empty();
        let vector = StlVector::<u8>::from_vec(bytes);
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), usize, *const ()) -> *const (), "?patch@Mod@geode@@QAE?AV?$Result@PAVPatch@geode@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@PAXABV?$vector@EV?$allocator@E@std@@@std@@@Z")(self.0, &result as *const _ as _, address, &vector as *const _ as _);
                        
            match result.to_result() {
                Ok(raw_patch) => Ok(Patch(raw_patch)),
                Err(stl_string) => Err(stl_string.to_string())
            }
        }
    }

    pub fn unpatch(&self, patch: &Patch) -> Result<(), String> {
        let result = GeodeResult::<*const (), StlString>::empty();
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "unpatch@Mod@geode@@QAE?AV?$Result@Umonostate@std@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@2@PAVPatch@2@@Z")(self.0, &result as *const _ as _, patch.0);
            
            match result.to_result() {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string())
            }
        }
    }
}

pub struct Patch(pub(crate) *const ());

//TODO: _spr
