use super::component_type::ComponentType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ArcheTypeHash(u64);

impl ArcheTypeHash {
    pub fn calc(component_types: &[&ComponentType]) -> ArcheTypeHash {
        let mut state = DefaultHasher::new();

        for component_type in component_types {
            component_type.hash(&mut state);
        }

        ArcheTypeHash(state.finish())
    }
}
