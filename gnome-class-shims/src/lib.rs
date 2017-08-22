pub extern crate libc;
pub extern crate gobject_sys;
pub extern crate glib_sys;

use glib_sys::{gpointer, GType};
use gobject_sys::{GObject, GObjectClass, GTypeClass, GTypeInstance};

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
pub unsafe trait GInstance: Sized {
    type Class: GClass;
    type Fields: GFields;

    fn get_type() -> GType;

    /// Extract the underlying `GObject` pointer. This will not
    /// increment the ref-count.
    ///
    /// Unsafe because `this` must be borrowed.
    unsafe fn to_gobject_ptr(this: *const Self) -> *mut GObject;

    /// Create an instance of `Self` from a `gpointer`. This will
    /// **not** increment the ref-count. Unsafe because caller
    /// must ensure that:
    ///
    /// 1. This is indeed a pointer of the correct type.
    /// 2. There is reference to surrender.
    unsafe fn from_gobject_ptr(g: *mut GObject) -> Self;

    unsafe fn borrow_gobject_ptr(g: &*mut GObject) -> &Self {
        ::std::mem::transmute(g)
    }

    /// Basically `Clone`
    fn clone_ref(&self) -> Self {
        unsafe {
            let ptr = Self::to_gobject_ptr(self);
            gobject_sys::g_object_ref(ptr);
            Self::from_gobject_ptr(ptr)
        }
    }

    /// Meant to be the body of a drop impl. Drops the ref-count on
    /// the internal ptr.
    unsafe fn drop_ref(&mut self) {
        let ptr = Self::to_gobject_ptr(self);
        gobject_sys::g_object_unref(ptr);
    }
}

/// Represents the actual fields of a GObject.
///
/// Unsafe contract: implementing this trait asserts that any time you
/// have a `&Self` reference, it is pointing at the fields of a
/// GObject. In other words, that you cannot instantiate `Self` on the
/// stack or in any way other than GObject-new.
pub unsafe trait GFields {
    type Instance: GInstance;

    fn as_instance<'r>(r: &'r &Self) -> &'r Self::Instance {
        // This is safe because we know that `*r` must actually
        // be a `*mut GObject`.
        unsafe { ::std::mem::transmute(r) }
    }
}

/// Represents a type that is a gnome class.
pub unsafe trait GClass {
    type Instance: GInstance;
}

/// Represents a class that is a subclass of some other gnome class.
pub unsafe trait GSubclass: GClass {
    type ParentClass: GClass;
}

/// Given something that must be a `GObject`, return the class of this
/// gobject.
pub fn get_class<T: GInstance>(this: &T) -> &T::Class {
    unsafe {
        let this: *mut GObject = T::to_gobject_ptr(this);
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

pub struct GObjectRef {
    v: *mut GObject
}

unsafe impl GInstance for GObjectRef {
    type Class = GObjectClass;
    type Fields = GObject;

    fn get_type() -> GType {
        unsafe {
            gobject_sys::g_object_get_type()
        }
    }

    unsafe fn to_gobject_ptr(this: *const GObjectRef) -> *mut GObject {
        (*this).v
    }

    unsafe fn from_gobject_ptr(v: *mut GObject) -> GObjectRef {
        GObjectRef { v: v }
    }
}

unsafe impl GFields for GObject {
    type Instance = GObjectRef;
}

unsafe impl GClass for GObjectClass {
    type Instance = GObjectRef;
}

/// An intentionally unconstructable zero-sized type.  This is used by
/// the GObject integration code to declare types that can never be
/// manually constructed by an end-user.
pub struct NoConstruct {
    _dummy: ()
}
