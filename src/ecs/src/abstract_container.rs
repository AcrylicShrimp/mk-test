use downcast_rs::{impl_downcast, Downcast};

pub trait AbstractContainer: Downcast {
	fn len(&self) -> usize;
}

impl_downcast!(AbstractContainer);
