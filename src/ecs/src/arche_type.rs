use super::abstract_container::AbstractContainer;
use super::arche_type_hash::ArcheTypeHash;
use super::arche_type_iter::ArcheTypeIter;
use super::component::Component;
use super::component_tuple::ComponentTuple;
use super::component_type::ComponentType;
use super::container::Container;
use super::container_factory::ContainerFactory;
use super::entity::Entity;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::slice::IterMut;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ArcheTypeError {
    InvalidComponentType(ComponentType),
}

pub struct ArcheType {
    hash: ArcheTypeHash,
    entities: Vec<Entity>,
    entity_indices: HashMap<Entity, usize>,
    components: HashMap<ComponentType, Box<dyn AbstractContainer>>,
    component_types: HashSet<ComponentType>,
    component_added_arche_types: HashMap<ComponentType, ArcheTypeHash>,
    component_removed_arche_types: HashMap<ComponentType, ArcheTypeHash>,
}

impl ArcheType {
    pub fn new(component_types: Vec<(ComponentType, Box<dyn ContainerFactory>)>) -> ArcheType {
        let inner_component_types = HashSet::from_iter(
            component_types
                .iter()
                .map(|(component_type, _)| component_type)
                .cloned(),
        );

        ArcheType {
            hash: ArcheTypeHash::calc(
                &component_types
                    .iter()
                    .map(|(component_type, _)| component_type)
                    .collect::<Vec<_>>(),
            ),
            entities: Vec::new(),
            entity_indices: HashMap::new(),
            components: HashMap::from_iter(
                component_types
                    .into_iter()
                    .map(|(component_type, factory)| (component_type, factory.new_container())),
            ),
            component_types: inner_component_types,
            component_added_arche_types: HashMap::new(),
            component_removed_arche_types: HashMap::new(),
        }
    }

    pub fn hash(&self) -> &ArcheTypeHash {
        &self.hash
    }

    pub fn has_components(&self, component_types: &HashSet<ComponentType>) -> bool {
        component_types.is_subset(&self.component_types)
    }

    pub fn couple_arche_type_added(
        &mut self,
        component_type: ComponentType,
        arche_type_hash: ArcheTypeHash,
    ) {
        self.component_added_arche_types
            .insert(component_type, arche_type_hash);
    }

    pub fn couple_arche_type_removed(
        &mut self,
        component_type: ComponentType,
        arche_type_hash: ArcheTypeHash,
    ) {
        self.component_removed_arche_types
            .insert(component_type, arche_type_hash);
    }

    pub fn decouple_arche_type(&mut self, hash: ArcheTypeHash) {
        self.component_added_arche_types
            .retain(|_, &mut other_hash| other_hash == hash);
        self.component_removed_arche_types
            .retain(|_, &mut other_hash| other_hash == hash);
    }

    pub fn add_entity<T: ComponentTuple>(
        &mut self,
        entity_id: u64,
        components: T,
    ) -> Result<Entity, ArcheTypeError> {
        let entity = Entity::new(entity_id);
        self.entity_indices.insert(entity, self.entities.len());
        self.entities.push(entity);
        components.add_entity_to(&mut self.components)?;
        Ok(entity)
    }
}

macro_rules! arche_type_iter_impls {
    (@next) => {};
    (@next $first:ident, $($rest:ident,)*) => {
        arche_type_iter_impls!($($rest,)*);
    };
    ($($components:ident,)*) => {
        impl<'a, $($components: Component,)*> ArcheTypeIter<'a, ($(IterMut<'a, $components>,)*)> for ArcheType {
            fn iter_mut(&'a mut self) -> ($(IterMut<'a, $components>,)*) {
                #[allow(unused_unsafe)]
                unsafe {
                    #[allow(non_snake_case)]
                    let ($($components,)*) = ($(self.components.get_mut(&ComponentType::of::<$components>()).unwrap() as *mut Box<dyn AbstractContainer>,)*);
                    ($((&mut *$components)
                        .downcast_mut::<Container<$components>>()
                        .unwrap()
                        .iter_mut(),)*)
                }
            }
        }

        arche_type_iter_impls!(@next $($components,)*);
    };
    () => {};
}

arche_type_iter_impls!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31,
);
