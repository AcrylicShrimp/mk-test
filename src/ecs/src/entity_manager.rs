use super::arche_type::ArcheType;
use super::arche_type_hash::ArcheTypeHash;
use super::component_tuple::ComponentTuple;
use super::component_type::ComponentType;
use super::entity::Entity;
use std::collections::{HashMap, HashSet};

pub struct EntityManager {
    arche_types: HashMap<ArcheTypeHash, ArcheType>,
    entities: Vec<ArcheTypeHash>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            arche_types: HashMap::new(),
            entities: Vec::new(),
        }
    }

    pub fn arche_types<'e>(
        &'e mut self,
        component_types: &HashSet<ComponentType>,
    ) -> Vec<&'e mut ArcheType> {
        self.arche_types
            .iter_mut()
            .filter(|(_, arche_type)| arche_type.has_components(component_types))
            .map(|(_, arche_type)| arche_type)
            .collect()
    }

    pub fn new_entity<T: ComponentTuple>(&mut self, components: T) -> Entity {
        let entity_id = self.entities.len() as u64;
        let hash = components.arche_type_hash();
        self.entities.push(hash);
        let arche_type = match self.arche_types.get_mut(&hash) {
            Some(arche_type) => arche_type,
            None => self
                .arche_types
                .entry(hash)
                .or_insert(components.arche_type()),
        };
        arche_type.add_entity(entity_id, components).unwrap()
    }
}

impl Default for EntityManager {
    fn default() -> EntityManager {
        EntityManager::new()
    }
}

// macro_rules! system_impls {
//     (@next) => {};
//     (@next $first:ident, $($rest:ident,)*) => {
//         system_impls!($($rest,)*);
//     };
//     ($($components:ident,)*) => {
//         impl<'e, $($components: ComponentRef<'e>,)*> System<'e, ($($components,)*)> for EntityManager {
//             fn for_each(&'e mut self, each: impl Fn(($($components,)*))) {
//                 let component_types = HashSet::from_iter(vec![$(ComponentType::of::<$components::Component>(),)*]);
//                 let mut iterators = self
//                     .arche_types
//                     .iter_mut()
//                     .filter(|(_, arche_type)| arche_type.has_components(&component_types))
//                     .map(|(_, arche_type)| {
//                         <ArcheType as ArcheTypeIter<'e, ($(IterMut<'e, $components::Component>,)*)>>::iter_mut(
//                             arche_type,
//                         )
//                     })
//                     .collect::<Vec<_>>();

//                 loop {
//                     match iterators.last_mut() {
//                         Some(iterator) => {
//                             #[allow(non_snake_case)]
//                             let ($($components,)*) = iterator;

//                             #[allow(non_snake_case)]
//                             #[allow(irrefutable_let_patterns)]
//                             if let ($(Some($components),)*) = ($($components.next(),)*) {
//                                 each(($($components::from_component($components),)*));
//                             } else {
//                                 iterators.pop();
//                             }
//                         }
//                         None => break,
//                     }
//                 }
//             }
//         }

//         system_impls!(@next $($components,)*);
//     };
//     () => {};
// }

// system_impls!(
//     T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
//     T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31,
// );
