use super::abstract_container::AbstractContainer;
use super::component_type::ComponentType;
use super::container::Container;
use std::any::Any;

pub trait Component: 'static + Sized {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_of(&self) -> ComponentType;
    fn container(&self) -> Box<dyn AbstractContainer>;
}

impl<T: 'static> Component for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_of(&self) -> ComponentType {
        ComponentType::of::<T>()
    }

    fn container(&self) -> Box<dyn AbstractContainer> {
        Box::new(Container::<T>::new())
    }
}
