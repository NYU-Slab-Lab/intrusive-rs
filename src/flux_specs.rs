//! Flux specifications for standard library functions that flux-core and
//! flux-alloc do not cover. These are trusted: each one only states a
//! guarantee or safety requirement documented by the standard library.

use flux_rs::attrs::*;

// Calling this on `None` is undefined behavior.
#[extern_spec(core::option)]
impl<T> Option<T> {
    #[spec(fn(Option<T>[true]) -> T)]
    unsafe fn unwrap_unchecked(self) -> T;
}

// `into_raw` returns the box's non-null allocation pointer, and `from_raw`
// requires a pointer previously returned by `into_raw`.
#[cfg(feature = "alloc")]
#[extern_spec(alloc::boxed)]
impl<T: ?Sized> Box<T> {
    #[spec(fn(b: Box<T>) -> *mut{p: p.addr != 0} T)]
    fn into_raw(b: Self) -> *mut T;

    #[spec(fn(raw: *mut{p: p.addr != 0} T) -> Box<T>)]
    unsafe fn from_raw(raw: *mut T) -> Self;
}

// As for `Box`, the pointer refers to the (non-null) shared value.
#[cfg(feature = "alloc")]
#[extern_spec(alloc::rc)]
impl<T: ?Sized> Rc<T> {
    #[spec(fn(this: Rc<T>) -> *const{p: p.addr != 0} T)]
    fn into_raw(this: Self) -> *const T;

    #[spec(fn(ptr: *const{p: p.addr != 0} T) -> Rc<T>)]
    unsafe fn from_raw(ptr: *const T) -> Self;
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[extern_spec(alloc::sync)]
impl<T: ?Sized> Arc<T> {
    #[spec(fn(this: Arc<T>) -> *const{p: p.addr != 0} T)]
    fn into_raw(this: Self) -> *const T;

    #[spec(fn(ptr: *const{p: p.addr != 0} T) -> Arc<T>)]
    unsafe fn from_raw(ptr: *const T) -> Self;
}
