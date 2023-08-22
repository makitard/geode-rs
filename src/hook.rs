use crate::tulip::{
    AbstractFunction, AbstractType, HandlerMetadata, HookMetadata, TulipConvention,
};

use crate::stl::{StlSharedPtr, StlString, StlVector};

pub struct Hook(pub(crate) *const ());

impl Hook {
    //TODO: learn how fn generics work and get the AbstractType automatically
    ///Creates a new Hook.
    ///For more information please visit <https://docs.geode-sdk.org>
    ///# Examples
    ///```
    ///use geode::prelude::*;
    ///
    ///
    /// unsafe extern "C" MenuLayer_onMoreGames_hook(this: *const (), sender: *const ()) {
    ///    log(Severity::Info, format!("Hello from {}:{}", file!(), line!()));
    //TODO: make this better
    ///    std::mem::transmute::<_, unsafe extern "thiscall" fn(*const (), *const ())>(
    ///        geode::utils::base() + 0x1919c0
    ///    )(this, sender);
    ///}
    ///
    ///geode::entry! {
    ///    let hook = Hook::new(
    ///        Mod::get(),
    ///        geode::utils::base() + 0x1919c0, //Address of MenuLayer::onMoreGames
    ///        MenuLayer_onMoreGames_hook,
    ///        tulip::TulipConvention::Thiscall,
    ///        tulip::ATY_VOID,
    ///        vec![tulip::ATY_PTR, tulip::ATY_PTR]
    ///     );
    ///
    ///    Mod::get().add_hook(hook);
    ///    //No need to call Mod::enable_hook because the hook gets enabled by default.
    ///}
    ///```
    pub fn new(
        owner: &crate::mod_::Mod,
        address: usize,
        detour: usize,
        display_name: &str,
        convention: TulipConvention,
        return_type: AbstractType,
        parameters: Vec<AbstractType>,
    ) -> Self {
        let hook_metadata = HookMetadata { m_priority: 0 };

        let mut m_convention = StlSharedPtr::empty();

        unsafe {
            geo_fn!(unsafe extern "C" fn(*mut StlSharedPtr, crate::tulip::TulipConvention), "?createConvention@hook@geode@@YA?AV?$shared_ptr@VCallingConvention@hook@tulip@@@std@@W4TulipConvention@1tulip@@@Z")(&mut m_convention as _, convention);
        }

        let handler_metadata = HandlerMetadata {
            m_convention,
            m_function: AbstractFunction {
                m_return: return_type,
                m_args: StlVector::<AbstractType>::from_vec(parameters),
            },
        };

        let display_name = StlString::from_str(display_name);

        unsafe {
            Self(
                geo_fn!(unsafe extern "C" fn(*const (), *const (), *const (), *const (), *const (), *const ()) -> *const (), "?create@Hook@geode@@SAPAV12@PAVMod@2@PAX1ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@ABVHandlerMetadata@hook@tulip@@ABVHookMetadata@78@@Z")(owner.0, address as _, detour as _, &display_name as *const _ as _, &handler_metadata as *const _ as _, &hook_metadata as *const _ as _)
            )
        }
    }
}
