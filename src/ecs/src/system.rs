use super::arche_type::ArcheType;
use super::arche_type_iter::ArcheTypeIter;
use super::component_ref::ComponentRef;
use super::component_ref_tuple::ComponentRefTuple;
use super::component_type::ComponentType;
use super::entity_manager::EntityManager;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::slice::IterMut;

pub struct System<'t, 's: 't, T: ComponentRefTuple<'t>> {
    component_types: HashSet<ComponentType>,
    executor: Box<dyn FnMut(T) + 's>,
    lifetimes: PhantomData<&'t &'s T>,
}

macro_rules! system_impls {
    (@next) => {};
    (@next $first:ident, $($rest:ident,)*) => {
        system_impls!($($rest,)*);
    };
    ($($components:ident,)*) => {
        impl<'t, 's: 't, 'e: 't, $($components: ComponentRef<'t>,)*> System<'t, 's, ($($components,)*)> {
            pub fn new<F: FnMut(($($components,)*)) + 's>(executor: F) -> Self {
                System {
                    component_types: HashSet::from_iter(vec![$(ComponentType::of::<$components::Component>(),)*]),
                    executor: Box::new(executor),
                    lifetimes: PhantomData,
                }
            }

            pub fn execute(&'s mut self, entity_manager: &'e mut EntityManager) {
                let mut iterators = entity_manager.arche_types(&self.component_types).into_iter().map(|arche_type|
                    <ArcheType as ArcheTypeIter<'t, ($(IterMut<'t, $components::Component>,)*)>>::iter_mut(
                        arche_type,
                    )
                ).collect::<Vec<_>>();

                loop {
                    match iterators.last_mut() {
                        Some(iterator) => {
                            #[allow(non_snake_case)]
                            let ($($components,)*) = iterator;

                            #[allow(non_snake_case)]
                            #[allow(irrefutable_let_patterns)]
                            if let ($(Some($components),)*) = ($($components.next(),)*) {
                                (*self.executor)(($($components::from_component($components),)*));
                            } else {
                                iterators.pop();
                            }
                        }
                        None => break,
                    }
                }
            }
        }

        system_impls!(@next $($components,)*);
    };
    () => {};
}

system_impls!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31,
);
