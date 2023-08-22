
pub trait EventListenerProtocol {
    unsafe extern "thiscall" fn handle(_this: *const (), event: *const ()) -> ListenerResult;
}

unsafe extern "thiscall" fn get_pool(this: *const CEventListenerProtocol) -> *const () {
    (*this).m_pool
}

unsafe extern "thiscall" fn destructor(_this: *const ()) {}

#[repr(C)]
struct CEvent {
    virtual_table: *const [*const (); 2],
    sender: *const (), //Mod
}

#[repr(i32)]
pub enum ListenerResult {
    Propagate,
    Stop
}

struct CEventListenerProtocol {
    virtual_table: *const [*const (); 3],
    m_pool: *const ()
}

impl CEventListenerProtocol {
    unsafe fn from_generic<T: EventListenerProtocol>() -> Self {
        let virtual_table: [*const (); 3] = [get_pool as _, T::handle as _, destructor as _];
        let raw_virtual_table = &virtual_table as *const _;
        std::mem::forget(virtual_table);

        Self {
            virtual_table: raw_virtual_table,
            m_pool: 0 as _
        }
    }
}

pub struct DefaultEventListenerPool(pub(crate) *const ());

impl DefaultEventListenerPool {
    pub fn get() -> Self {
        return Self(unsafe { geo_fn!(unsafe extern "C" fn() -> *const (), "?get@DefaultEventListenerPool@geode@@SAPAV12@XZ")() })
    }

    pub fn add<T: EventListenerProtocol>(&self) -> EventListener {
        unsafe {
            let c_listener = CEventListenerProtocol::from_generic::<T>();
            let raw_listener = &c_listener as *const _;
            std::mem::forget(c_listener);

            geo_fn!(unsafe extern "thiscall" fn(*const (), *const ()) -> bool, "?add@DefaultEventListenerPool@geode@@UAE_NPAVEventListenerProtocol@2@@Z")(self.0, raw_listener as _);
        
            EventListener(raw_listener as _)
        }
    }

    pub fn remove(&self, listener: EventListener) {
        unsafe {
            geo_fn!(unsafe extern "thiscall" fn(*const (), *const ()), "?remove@DefaultEventListenerPool@geode@@UAEXPAVEventListenerProtocol@2@@Z")(self.0, listener.0);
        }
    }
}

pub struct EventListener(pub(crate) *const ());