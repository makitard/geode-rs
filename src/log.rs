#![allow(non_snake_case)]

use crate::stl::{StlString, StlTimepoint, StlVector};

//TODO: macros

pub fn log(severity: Severity, msg: &str) {
    let component = ComponentBaseString::new(StlString::from_str(msg));
    let log = Log {
        m_sender: crate::mod_::Mod::get().0,
        m_time: StlTimepoint::since_epoch(),
        m_components: StlVector::<*const ComponentTrait>::from_vec(vec![
            &component as *const _ as _,
        ]),
        m_severity: severity,
    };

    unsafe {
        geo_fn!(
            unsafe extern "C" fn(*const ()),
            "?push@Logger@log@geode@@SAX$$QAVLog@23@@Z"
        )(&log as *const _ as _)
    };
}

#[repr(i32)]
pub enum Severity {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

#[repr(C)]
struct Log {
    m_sender: *const (), //Mod*
    m_time: StlTimepoint,
    m_components: StlVector<*const ComponentTrait>,
    m_severity: Severity,
}

const _: () = assert!(
    std::mem::size_of::<Log>() == 32,
    "The Log struct is not 32 bytes in size! Make sure you are compiling for i686."
);

#[repr(C)]
struct ComponentTrait {
    virtual_table: *const [*const (); 2],
}

//not dealing with generics lol
#[repr(C)]
struct ComponentBaseString {
    super_: ComponentTrait,
    m_item: StlString,
}

impl ComponentBaseString {
    pub fn new(s: StlString) -> Self {
        static mut COMPONENTBASE_VTABLE: [*const (); 2] =
            [ComponentBase_destructor as _, ComponentBase_toString as _];

        Self {
            super_: ComponentTrait {
                virtual_table: unsafe { &COMPONENTBASE_VTABLE as _ },
            },
            m_item: s,
        }
    }
}

unsafe extern "thiscall" fn ComponentBase_destructor(_this: *const ComponentBaseString) {}

unsafe extern "thiscall" fn ComponentBase_toString(
    this: *const ComponentBaseString,
    out: *mut StlString,
) -> *const StlString {
    *out = (*this).m_item;
    &(*this).m_item as _
}
