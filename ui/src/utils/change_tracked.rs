use std::ops;

pub struct ChangeTracked<'a, T>
where
    T: Clone + Eq,
{
    initial: &'a T,
    current: T,
}

impl<'a, T> ChangeTracked<'a, T>
where
    T: Clone + Eq,
{
    pub fn new(value: &'a T) -> Self {
        Self {
            initial: value,
            current: value.clone(),
        }
    }

    pub fn is_changed(&self) -> bool {
        self.initial != &self.current
    }

    pub fn changed(self) -> Option<T> {
        if self.is_changed() {
            Some(self.current)
        } else {
            None
        }
    }

    pub fn map_change<F>(self, on_change: F)
    where
        F: FnOnce(T),
    {
        if self.is_changed() {
            on_change(self.current);
        }
    }
}

impl<'a, T> ops::Deref for ChangeTracked<'a, T>
where
    T: Clone + Eq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<'a, T> ops::DerefMut for ChangeTracked<'a, T>
where
    T: Clone + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}
