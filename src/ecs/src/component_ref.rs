use super::component::Component;

pub trait ComponentRef<'a>: 'a + Sized {
    type Component: Component;
    fn from_component(component: &'a mut Self::Component) -> Self;
}

impl<'a, T: Component> ComponentRef<'a> for &'a T {
    type Component = T;
    fn from_component(component: &'a mut Self::Component) -> Self {
        component
    }
}

impl<'a, T: Component> ComponentRef<'a> for &'a mut T {
    type Component = T;
    fn from_component(component: &'a mut Self::Component) -> Self {
        component
    }
}
