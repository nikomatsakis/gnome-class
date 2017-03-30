use g::*;
pub use gobject_sys::{GObject, GObjectClass};

unsafe impl GInstance for GObject {
    type Class = GObjectClass;
}

unsafe impl GClass for GObjectClass {
    type Instance = GObject;
}

