use crate::hook::Hook;
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

    //TODO: return the actual result (and recreate geode::utils::impl::Result<T>)
    pub fn add_hook(&mut self, hook: &Hook) {
        let result = [0u8; 16]; //idk the actual size of this so TODO
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?addHook@Mod@geode@@QAE?AV?$Result@PAVHook@geode@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0)
        };
    }

    pub fn enable_hook(&mut self, hook: &Hook) {
        let result = [0u8; 16];
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?enableHook@Mod@geode@@QAE?AV?$Result@Umonostate@std@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0)
        };
    }

    pub fn disable_hook(&mut self, hook: &Hook) {
        let result = [0u8; 16];
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const (), *const ()) -> *const (), "?disableHook@Mod@geode@@QAE?AV?$Result@Umonostate@std@@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@2@@2@PAVHook@2@@Z")(self.0, &result as *const _ as _, hook.0)
        };
    }
}

//TODO: _spr