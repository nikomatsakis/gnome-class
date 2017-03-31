pub extern crate gobject_sys;
pub extern crate glib_sys;

use glib_sys::{gpointer, GType};
use gobject_sys::{GObject, GObjectClass, GTypeClass, GTypeInstance};
use std::ops::Deref;

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
/// implement `Clone` and `GInstance` together, since then safe
/// code could take an `&Self` and produce a `Self` that is not stored
/// in a gobject.
pub unsafe trait GInstance {
    type Class: GClass;
    fn get_type() -> GType;
}

/// Represents a type that is a gnome class.
pub unsafe trait GClass {
    type Instance: GInstance;
}

/// Represents a class that is a subclass of some other gnome class.
pub unsafe trait GSubclass: GClass {
    type ParentClass: GClass;
}

/// A reference to a `GObject`; the `T` is the subtype of `GObject`.
pub struct G<T: GInstance> {
    data: *const T
}

/// Convert `p`, which is a pointer to the contents of some `GObject`, into
/// a `*mut GObject`, for use with the gtk-rs APIs.
///
/// NB: There is a subtle interaction here in the case where `T` is a
/// trait type (e.g., the trait type representing a class). In that
/// case, we take in a fat pointer (`*const Trait`) and return a thin
/// pointer (`*mut GObject`) representing just the data itself,
/// stripped of its vtable.
pub fn to_gobject_ptr<T: GInstance>(p: *const T) -> *mut GObject {
    p as *mut GObject
}

/// Given a valid pointer to a `GInstance`, we can convert
/// this into an owned `G<>` reference by incrementing the
/// reference count.
///
/// This is safe because:
///
/// - The trait requires that each instance of `self` must be
///   inside some gobject allocation.
/// - Having an `&Self` instance means that this gobject allocation
///   must have a valid ref-count spanning this call.
pub fn to_object_ref<T: GInstance>(p: &T) -> &G<T> {
    unsafe {
        let p = p as *const T;
        let p = p as *const G<T>;
        &*p
    }
}

/// Given something that must be a `GObject`, return the class of this
/// gobject.
pub fn get_class<T: GInstance>(this: &T) -> &T::Class {
    unsafe {
        let this: *mut GObject = to_gobject_ptr(this);
        let type_instance: *const GTypeInstance = &(*this).g_type_instance;

        /// I am a horrible monster and I pray for death: the first
        /// field of `GTypeInstance` has type `*mut GTypeClass`, but it
        /// is private in the `gobject_sys` crate. Therefore, we cast
        /// this pointer to a pointer to the first field and read from
        /// it.
        #[repr(C)]
        struct GTypeInstance1 {
            g_class: *mut GTypeClass
        }

        let type_instance = type_instance as *const GTypeInstance1;
        let klass: *mut GTypeClass = (*type_instance).g_class;
        let klass: *const T::Class = klass as *const T::Class;
        &*klass
    }
}

pub fn get_parent_class<T: GSubclass>(this: &T) -> &T::ParentClass {
    unsafe {
        let this: *const T = this;
        let parent_class = gobject_sys::g_type_class_peek_parent(this as gpointer);
        let parent_class = parent_class as *mut T::ParentClass;
        &*parent_class
    }
}

impl<T: GInstance> G<T> {
    pub unsafe fn new(data: *const T) -> G<T> {
        G { data: data }
    }
}

impl<T: GInstance> Clone for G<T> {
    fn clone(&self) -> Self {
        unsafe {
            gobject_sys::g_object_ref(to_gobject_ptr(self.data));
            G::new(self.data)
        }
    }
}

impl<T: GInstance> Deref for G<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.data)
        }
    }
}

impl<T: GInstance> Drop for G<T> {
    fn drop(&mut self) {
        unsafe {
            let ptr = to_gobject_ptr(self.data);
            gobject_sys::g_object_unref(ptr);
        }
    }
}

unsafe impl GInstance for GObject {
    type Class = GObjectClass;

    fn get_type() -> GType {
        unsafe {
            gobject_sys::g_object_get_type()
        }
    }
}

unsafe impl GClass for GObjectClass {
    type Instance = GObject;
}

/// An intentionally unconstructable zero-sized type.  This is used by
/// the GObject integration code to declare types that can never be
/// manually constructed by an end-user.
pub struct NoConstruct {
    _dummy: ()
}
