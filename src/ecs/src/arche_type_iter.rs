use super::component_iter_tuple::ComponentIterTuple;

pub trait ArcheTypeIter<'a, T: ComponentIterTuple<'a>> {
    fn iter_mut(&'a mut self) -> T;
}
