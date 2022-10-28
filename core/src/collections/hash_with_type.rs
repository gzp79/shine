use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
};

/// If the content is the same but types are different, the default hasher would generate the same value.
/// This helper ensures the type is also considered.
/// For example hashing a slotmap key with the same version and index but for different type would create the same hash.
pub struct HashWithType<Id>(pub Id)
where
    Id: Any + Hash;

impl<Id> Hash for HashWithType<Id>
where
    Id: Any + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        TypeId::of::<Self>().hash(state)
    }
}

impl<Id> From<Id> for HashWithType<Id>
where
    Id: Any + Hash,
{
    fn from(id: Id) -> Self {
        HashWithType(id)
    }
}
