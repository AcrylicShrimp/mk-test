pub trait Component {}

impl<T: 'static> Component for T {}

pub trait ComponentAsParam {}

impl<T: Component> ComponentAsParam for &T {}
impl<T: Component> ComponentAsParam for &mut T {}

pub trait ComponentAsParamTuple {}

impl<T0: ComponentAsParam> ComponentAsParamTuple for (T0,) {}
impl<T0: ComponentAsParam, T1: ComponentAsParam> ComponentAsParamTuple for (T0, T1) {}

pub trait Resource {}

impl Resource for () {}
impl<T0: 'static> Resource for (&T0,) {}
impl<T0: 'static, T1: 'static> Resource for (&T0, &T1) {}

pub trait System<C: ComponentAsParamTuple, R: Resource> {
	fn execute(&self, components: C, resources: R);
}

impl<C: ComponentAsParamTuple, R: Resource, F> System<C, R> for F
where
	F: Fn(C, R),
{
	fn execute(&self, components: C, resources: R) {
		self(components, resources);
	}
}
