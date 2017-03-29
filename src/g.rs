use gobject_sys::{self, GObject};
use std::ops::Deref;

/// A reference to a `GObject`; the `T` is the subtype of `GObject`.
pub struct G<T: GObjectContents + ?Sized> {
    data: *const T
}

/// A trait that is implemented for all things that may be gobjects.
/// This trait is unsafe because implementing it implies validating
/// certain requirements:
///
/// - The data begins, at offset 0, with the `GObject` fields.
/// - A `*mut Self` can be safely cast to a `gtypes::gpointer`.
/// - All instances of the type will be allocated in an actual
///   gobject (in particular, the methods of this trait assume
///   that the receiver must be allocated in an `GObject`).
///
/// Note that the final point means that it is incorrect to ever
/// implement `Clone` and `GObjectContents` together, since then safe
/// code could take an `&Self` and produce a `Self` that is not stored
/// in a gobject.
pub unsafe trait GObjectContents {
}

fn to_gobject_ptr<T: GObjectContents + ?Sized>(p: *const T) -> *mut GObject {
    p as *mut GObject
}

/// Given a valid pointer to a `GObjectContents`, we can convert
/// this into an owned `G<>` reference by incrementing the
/// reference count.
///
/// This is safe because:
///
/// - The trait requires that each instance of `self` must be
///   inside some gobject allocation.
/// - Having an `&Self` instance means that this gobject allocation
///   must have a valid ref-count spanning this call.
pub fn to_ref<T: GObjectContents + ?Sized>(p: &T) -> G<T> {
    unsafe {
        gobject_sys::g_object_ref(to_gobject_ptr(p));
        G::new(p)
    }
}

impl<T: GObjectContents + ?Sized> G<T> {
    pub unsafe fn new(data: *const T) -> G<T> {
        G { data: data }
    }
}

impl<T: GObjectContents + ?Sized> Clone for G<T> {
    fn clone(&self) -> Self {
        unsafe { to_ref(&*self.data) }
    }
}

impl<T: GObjectContents + ?Sized> Deref for G<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.data)
        }
    }
}

impl<T: GObjectContents + ?Sized> Drop for G<T> {
    fn drop(&mut self) {
        unsafe {
            let ptr = to_gobject_ptr(self.data);
            gobject_sys::g_object_unref(ptr);
        }
    }
}

