#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Entity(u64);

impl Entity {
    pub fn new(id: u64) -> Entity {
        Entity(id)
    }
}
