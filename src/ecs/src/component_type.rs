use super::component::Component;
use std::any::TypeId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ComponentType(TypeId);

impl ComponentType {
    pub fn of<T: Component>() -> Self {
        Self(TypeId::of::<T>())
    }
}
