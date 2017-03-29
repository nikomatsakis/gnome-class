use ptr::Ptr;

pub trait GObject {
    fn GObject(&self) -> &GObjectFields;
    fn GObjectPtr(&self) -> Ptr<GObject>;
}

pub struct GObjectFields {
    dummy: ()
}

impl GObjectFields {
    pub fn new() -> Self {
        GObjectFields { dummy: () }
    }
}
