use super::system::{ComponentAsParamTuple, Resource, System};

pub struct SystemScheduler {}

impl SystemScheduler {
	pub fn new<C: ComponentAsParamTuple, R: Resource, S: System<C, R>>(
		system: S,
	) -> SystemScheduler {
		SystemScheduler {}
	}
}
