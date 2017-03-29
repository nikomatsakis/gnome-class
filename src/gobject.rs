use gobject_sys;
use ptr::Ptr;
use std::mem;

pub trait GObject {
    fn GObject(&self) -> &GObjectFields;
    fn GObjectPtr(&self) -> Ptr<GObject>;
}

pub struct GObjectFields {
    fields: gobject_sys::GObject
}

impl GObjectFields {
    pub fn new() -> Self {
        unsafe { mem::zeroed::<GObjectFields>() }
    }
}
