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

/// Convert `p`, which is a pointer to the contents of some `GObject`, into
/// a `*mut GObject`, for use with the gtk-rs APIs.
///
/// NB: There is a subtle interaction here in the case where `T` is a
/// trait type (e.g., the trait type representing a class). In that
/// case, we take in a fat pointer (`*const Trait`) and return a thin
/// pointer (`*mut GObject`) representing just the data itself,
/// stripped of its vtable.
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
///
///
/// NB: In principle, the signature of this method could be `&T ->
/// &G<T>`, which would be nice since we would forego one
/// ref-count. However, that doesn't work because, in the case where
/// `T` is a trait, we would be taking in a fat pointer and returning
/// a thin pointer that is supposed to be a pointer to the fat
/// pointer.
///
/// Open question: It might be possible to take an `&&T` and return an
/// `&G<T>` so as to avoid this.
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

