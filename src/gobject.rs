use g::GObjectContents;
use gobject_sys;
use std::mem;

pub trait GObject: 'static + GObjectContents {
    fn GObject(&self) -> &GObjectFields;
}

pub struct GObjectFields {
    fields: gobject_sys::GObject
}

impl GObjectFields {
    pub fn new() -> Self {
        unsafe { mem::zeroed::<GObjectFields>() }
    }
}
