use super::abstract_container::AbstractContainer;
use super::arche_type::{ArcheType, ArcheTypeError};
use super::arche_type_hash::ArcheTypeHash;
use super::component::Component;
use super::component_type::ComponentType;
use super::container::Container;
use std::collections::HashMap;

pub trait ComponentTuple: 'static + Sized {
    fn arche_type(&self) -> ArcheType;
    fn arche_type_hash(&self) -> ArcheTypeHash;
    fn add_entity_to(
        self,
        components: &mut HashMap<ComponentType, Box<dyn AbstractContainer>>,
    ) -> Result<(), ArcheTypeError>;
}

macro_rules! component_tuple_impls {
    (@impl next) => {};
    (@impl next $first:ident, $($rest:ident,)*) => {
        component_tuple_impls!($($rest,)*);
    };
    (@impl $components:ident, ) => {
        return Ok(());
    };
    (@impl $components:ident, $first:ident, $($rest:ident,)*) => {
        match $components.get_mut(&ComponentType::of::<$first>()) {
            Some(container) => {
                container
                    .downcast_mut::<Container<$first>>()
                    .unwrap()
                    .insert($first);
            }
            None => return Err(ArcheTypeError::InvalidComponentType(ComponentType::of::<$first>())),
        }
        component_tuple_impls!(@impl $components, $($rest,)*);
    };
    ($($components:ident,)*) => {
        impl<$($components: Component,)*> ComponentTuple for ($($components,)*) {
            fn arche_type(&self) -> ArcheType {
                ArcheType::new(vec![$((ComponentType::of::<$components>(), Box::new(|| -> Box<dyn AbstractContainer> { Box::new(Container::<$components>::new()) })), ) *])
            }

            fn arche_type_hash(&self) -> ArcheTypeHash {
                ArcheTypeHash::calc(&[$(&ComponentType::of::<$components>(), ) *])
            }

            fn add_entity_to(
                self,
                #[allow(unused_variables)]
                components: &mut HashMap<ComponentType, Box<dyn AbstractContainer>>,
            ) -> Result<(), ArcheTypeError> {
                #[allow(non_snake_case)]
                let ($($components,)*) = self;
                component_tuple_impls!(@impl components, $($components,)*);
            }
        }

        component_tuple_impls!(@impl next $($components,)*);
    };
    () => {};
}

component_tuple_impls!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31,
);
