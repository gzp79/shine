use std::{
    any::{Any, TypeId},
    collections::{hash_map::Entry, HashMap},
};

/// Allow store an instance form each type. And provides generic
/// accessors for them.
#[derive(Default)]
pub struct TypeMap {
    extensions: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    pub fn set<D: 'static>(&mut self, data: D) {
        let ty = TypeId::of::<D>();
        self.extensions.insert(ty, Box::new(data));
    }

    pub fn get<D: 'static>(&self) -> Option<&D> {
        let ty = TypeId::of::<D>();
        Some(self.extensions.get(&ty)?.downcast_ref::<D>().unwrap())
    }

    pub fn get_mut<D: 'static>(&mut self) -> Option<&mut D> {
        let ty = TypeId::of::<D>();
        Some(self.extensions.get_mut(&ty)?.downcast_mut::<D>().unwrap())
    }

    pub fn get_or_default<D: 'static + Default>(&mut self) -> &mut D {
        let ty = TypeId::of::<D>();
        match self.extensions.entry(ty) {
            Entry::Occupied(entry) => entry.into_mut().downcast_mut::<D>().unwrap(),
            Entry::Vacant(entry) => entry.insert(Box::<D>::default()).downcast_mut::<D>().unwrap(),
        }
    }
}
