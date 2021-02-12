use super::abstract_container::AbstractContainer;
use super::component::Component;
use std::slice::IterMut;

pub struct Container<T: Component> {
    components: Vec<T>,
}

impl<T: Component> Container<T> {
    pub fn new() -> Container<T> {
        Container {
            components: Vec::new(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.components.iter_mut()
    }

    pub fn insert(&mut self, component: T) {
        self.components.push(component);
    }
}

impl<T: Component> Default for Container<T> {
    fn default() -> Container<T> {
        Container::new()
    }
}

impl<T: Component> AbstractContainer for Container<T> {
    fn len(&self) -> usize {
        self.components.len()
    }
}
