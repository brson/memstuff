#![no_std]
#![allow(unused)]

pub mod c {
    use libc::{c_void, c_int, size_t};
    use crate::r;

    pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, num: size_t) {
        r::memcpy(dest as _, src as _, num as _)
    }

    pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, num: size_t) {
        r::memmove(dest as _, src as _, num as _)
    }

    pub unsafe extern "C" fn memcmp(ptr1: *const c_void, ptr2: *const c_void, num: size_t) -> c_int {
        r::memcmp(ptr1 as _, ptr2 as _, num as _) as _
    }

    pub unsafe extern "C" fn memchr(ptr: *const c_void, value: c_int, num: size_t) -> *const c_void {
        r::memchr(ptr as _, value as _, num as _) as _
    }

    pub unsafe extern "C" fn memset(ptr: *mut c_void, value: c_int, num: size_t) {
        r::memset(ptr as _, value as _, num as _)
    }
}

pub mod r {
    use crate::i;

    #[inline]
    pub unsafe fn memcpy(dest: *mut u8, src: *const u8, num: usize) {
        i::memcpy::memcpy(dest, src, num)
    }

    #[inline]
    pub unsafe fn memmove(dest: *mut u8, src: *const u8, num: usize) {
        i::memmove::memmove(dest, src, num)
    }

    #[inline]
    pub unsafe fn memcmp(ptr1: *const u8, ptr2: *const u8, num: usize) -> i8 {
        i::memcmp::memcmp(ptr1, ptr2, num)
    }

    #[inline]
    pub unsafe fn memchr(ptr: *const u8, value: u8, num: usize) -> *const u8 {
        i::memchr::memchr(ptr, value, num)
    }

    #[inline]
    pub unsafe fn memset(ptr: *mut u8, value: u8, num: usize) {
        i::memset::memset(ptr, value, num)
    }
}

pub mod i {
    pub use crate::memcpy_impl as memcpy;
    pub use crate::memmove_impl as memmove;
    pub use crate::memcmp_impl as memcmp;
    pub use crate::memchr_impl as memchr;
    pub use crate::memset_impl as memset;
}

#[doc(hidden)]
#[path = "memcpy.rs"]
pub mod memcpy_impl;
#[doc(hidden)]
#[path = "memmove.rs"]
pub mod memmove_impl;
#[doc(hidden)]
#[path = "memcmp.rs"]
pub mod memcmp_impl;
#[doc(hidden)]
#[path = "memchr.rs"]
pub mod memchr_impl;
#[doc(hidden)]
#[path = "memset.rs"]
pub mod memset_impl;
