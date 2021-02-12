use super::abstract_container::AbstractContainer;

pub trait ContainerFactory {
    fn new_container(&self) -> Box<dyn AbstractContainer>;
}

impl<F> ContainerFactory for F
where
    F: Fn() -> Box<dyn AbstractContainer>,
{
    fn new_container(&self) -> Box<dyn AbstractContainer> {
        self()
    }
}
