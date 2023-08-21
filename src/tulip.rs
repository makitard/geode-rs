use crate::stl::{StlSharedPtr, StlVector};

#[repr(C)]
pub struct HookMetadata {
    pub(crate) m_priority: i32,
}

#[repr(C)]
pub struct HandlerMetadata {
    pub(crate) m_convention: StlSharedPtr,
    pub(crate) m_function: AbstractFunction,
}

#[repr(C)]
pub struct AbstractFunction {
    pub(crate) m_return: AbstractType,
    pub(crate) m_args: StlVector<AbstractType>,
}

#[repr(C)]
pub struct AbstractType {
    pub(crate) m_size: usize,
    pub(crate) m_kind: AbstractTypeKind,
}

pub const ATY_VOID: AbstractType = AbstractType {
    m_size: 1,
    m_kind: AbstractTypeKind::Primitive,
};

pub const ATY_POINTER: AbstractType = AbstractType {
    m_size: std::mem::size_of::<*const ()>(),
    m_kind: AbstractTypeKind::Primitive,
};

//TODO: AbstractType and AbstractFunction from T
impl AbstractType {
    pub const fn new(m_size: usize, m_kind: AbstractTypeKind) -> Self {
        Self { m_size, m_kind }
    }
}

#[repr(u8)]
pub enum AbstractTypeKind {
    Primitive,
    FloatingPoint,
    Other,
}

#[repr(i32)]
pub enum TulipConvention {
    Default,
    Cdecl,
    Thiscall,
    Fastcall,
    Optcall,
    Membercall,
}
