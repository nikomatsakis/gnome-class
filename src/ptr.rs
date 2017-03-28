use std::sync;

pub type Ptr<T> = sync::Arc<T>;
pub type Weak<T> = sync::Weak<T>;
