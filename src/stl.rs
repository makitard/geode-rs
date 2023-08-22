//! MSVC C++ STL types

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StlString {
    _bx: StlStringContainer,
    _mysize: usize,
    _myres: usize,
}

impl StlString {
    pub fn from_str(s: &str) -> Self {
        let _bx = if s.len() < 16 {
            let mut str_ = std::ffi::CString::new(s).unwrap().as_bytes_with_nul().to_vec();
            str_.resize(16, 0);
            StlStringContainer {
                _buf: str_
                    .try_into()
                    .unwrap(),
            }
        } else {
            let cs = std::ffi::CString::new(s).unwrap();
            let _ptr = cs.as_ptr() as *mut u8;
            std::mem::forget(cs);
            StlStringContainer { _ptr }
        };

        Self {
            _bx,
            _mysize: s.len(),
            _myres: s.len(),
        }
    }

    pub unsafe fn to_string(self) -> String {
        std::ffi::CString::from_raw(if self._mysize < 16 { self._bx._buf.as_ptr() as _ } else { self._bx._ptr as _ }).to_string_lossy().to_string()
    }

    pub const fn empty() -> Self {
        Self {
            _bx: StlStringContainer { _buf: [0u8; 16] },
            _mysize: 0,
            _myres: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
union StlStringContainer {
    _buf: [u8; 16],
    _ptr: *mut u8,
}

#[repr(C)]
pub struct StlVector<T: ?Sized> {
    _myfirst: *const T,
    _mylast: *const T,
    _myend: *const T,
}

impl<T> StlVector<T> {
    pub fn from_vec(v: Vec<T>) -> Self {
        if v.is_empty() {
            Self {
                _myfirst: 0 as _,
                _mylast: 0 as _,
                _myend: 0 as _,
            }
        } else {
            let first = v.as_ptr();
            let last = first as usize + v.len() * std::mem::size_of::<T>();
            std::mem::forget(v);
            Self {
                _myfirst: first,
                _mylast: last as _,
                _myend: last as _,
            }
        }
    }

    pub const fn empty() -> Self {
        Self {
            _myfirst: 0 as _,
            _mylast: 0 as _,
            _myend: 0 as _,
        }
    }
}

#[repr(C)]
pub(crate) struct StlSharedPtr {
    _ptr: *const (),
    _rep: *const (),
}

impl StlSharedPtr {
    pub const fn empty() -> Self {
        Self {
            _ptr: 0 as _,
            _rep: 0 as _,
        }
    }
}

pub(crate) type StlTimepoint = StlDuration;

//this has a generic but geode only uses std::chrono::system_clock::time_point which is duration<long long, ratio<1, 10'000'000>>
#[repr(C)]
pub struct StlDuration {
    _rep: i64, //in nanoseconds / 100
}

impl StlDuration {
    pub fn from_duration(duration: std::time::Duration) -> Self {
        Self {
            _rep: (duration.as_nanos() as i64) / 100,
        }
    }

    pub fn since_epoch() -> Self {
        Self::from_duration(
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default(),
        )
    }
}

//not actually in the stl
#[repr(C)]
pub(crate) struct GeodeResult<T: Copy, E: Copy> {
    union_: GeodeResultUnion<T, E>,
    m_has_value: bool,
}

#[repr(C)]
pub(crate) union GeodeResultUnion<T: Copy, E: Copy> {
    m_value: T, //wrapped
    m_error: E,
    m_empty: () //IDK
}

impl<T: Copy, E: Copy> GeodeResult<T, E> {
    pub fn to_result(self) -> Result<T, E> {
        unsafe {
            if self.m_has_value {
                Ok(self.union_.m_value)
            } else {
                Err(self.union_.m_error)
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            union_: GeodeResultUnion {
                m_empty: ()
            },
            m_has_value: false
        }
    }
}