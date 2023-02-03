use std::ops::Deref;

pub enum OwnOrRef<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T> OwnOrRef<'a, T> {
    pub fn get(&self) -> &T {
        match self {
            OwnOrRef::Borrowed(t) => t,
            OwnOrRef::Owned(t) => t,
        }
    }

    pub fn into_owned(self) -> T
    where T: Clone
    {
        match self {
            OwnOrRef::Borrowed(t) => t.clone(),
            OwnOrRef::Owned(t) => t,
        }
    }
}

impl<'a, T> Deref for OwnOrRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> AsRef<T> for OwnOrRef<'a, T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}
