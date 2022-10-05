use downcast_rs::Downcast;
use std::{
    any::Any,
    mem::{self, MaybeUninit},
    ptr,
};

pub trait SmallAnyBoxLayout { 
    fn init() -> Self; 
    fn as_ptr(&self) -> *const u8;
    fn as_mut_ptr(&mut self) -> *mut u8;
}

#[macro_export]
/// A small helper to adjust small node layout in a more user friendly format.
macro_rules! define_layout {
    ($vis:vis $name:ident => $size:literal, $align:literal) => {
        #[repr(align($align))]
        $vis struct $name {
            data: [u8; $size],
        }

        impl $crate::collections::SmallAnyBoxLayout for $name { 
            fn init() -> Self { Self{ data: [0; $size] } } 
            fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }
            fn as_mut_ptr(&mut self) -> *mut u8 { self.data.as_mut_ptr() }
        }
    };
}

enum Inner<Store: SmallAnyBoxLayout> {
    Removed,
    Small(Store, usize, fn(&Store) -> &dyn Any, fn(&mut Store) -> &mut dyn Any, fn(&mut Store)),
    Big(Box<dyn Any>),
}

pub mod small_box_layout {
    define_layout!( pub S64_8 => 64, 8);
    define_layout!( pub S8_1 => 8, 1);
}

pub struct SmallAnyBox<Store: SmallAnyBoxLayout>(Inner<Store>);

impl<Store: SmallAnyBoxLayout> Drop for SmallAnyBox<Store> {
    fn drop(&mut self) {
        if let Inner::Small(store, _, _, _, drop) = &mut self.0 {            
            drop(store);
        }
    }
}

impl<Store: SmallAnyBoxLayout> SmallAnyBox<Store> {
    pub fn new<T: Downcast>(node: T) -> Self {
        let size = mem::size_of::<T>();
        assert!(size > 0);
        let align = mem::align_of::<T>();

        if size > mem::size_of::<Store>() || align > mem::align_of::<Store>() {
            // Big Node
            Self(Inner::Big(Box::new(node)))
        } else {            
            let mut space = Store::init();
            let ptr = space.as_mut_ptr();
            unsafe {
                let src = &node as *const T;
                ptr::copy_nonoverlapping(src as *const u8, ptr, size);                
            };
            mem::forget(node);
            
            let as_ref: fn(&Store) -> &dyn Any = |space| {
                let ptr = space.as_ptr() as *const T;
                unsafe { &*ptr}
            };
            let as_mut: fn(&mut Store) -> &mut dyn Any = |space| {
                let ptr = space.as_mut_ptr() as *mut T;
                unsafe { &mut *ptr}
            };
            let drop: fn(&mut Store) = |space| {
                let ptr = space.as_mut_ptr() as *mut T;
                unsafe { ptr::drop_in_place(ptr) };
            };
            
            Self(Inner::Small(space, size, as_ref, as_mut, drop))
        }
    }

    /// Return if object can be stored embedded in the structure.
    pub fn is_small(&self) -> bool {
        matches!(&self.0, Inner::Small(..))
    }

    /// Return if object is stored on the heap.
    pub fn is_big(&self) -> bool {
        matches!(&self.0, Inner::Big(..))
    }

    pub fn as_any(&self) -> &dyn Any {
        match &self.0 {
            Inner::Removed => unreachable!(),
            Inner::Small(space, _, as_ref, _, _) => as_ref(space),
            Inner::Big(bx) => bx,
        }
    }

    pub fn as_mut_any(&mut self) -> &mut dyn Any {
        match &mut self.0 {
            Inner::Removed => unreachable!(),
            Inner::Small(space, _, _, as_mut, _) => as_mut(space),
            Inner::Big(bx) => bx,
        }
    }

    pub fn as_ref<T: Downcast>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn as_mut<T: Downcast>(&mut self) -> Option<&mut T> {
        self.as_mut_any().downcast_mut::<T>()
    }

    pub fn take_as<T: Sized + Downcast>(mut self) -> Result<T, Self> {
        match mem::replace(&mut self.0, Inner::Removed) {
            Inner::Removed => unreachable!(),
            Inner::Small(space, size, as_ref, as_mut, drop) => {
                if as_ref(&space).is::<T>() {
                    let mut node = MaybeUninit::<T>::uninit();
                    let ptr = space.as_ptr();
                    unsafe { ptr::copy_nonoverlapping(ptr, node.as_mut_ptr() as *mut u8, size) };
                    Ok(unsafe { node.assume_init() })
                } else {
                    Err(Self(Inner::Small(space, size, as_ref, as_mut, drop)))
                }
            }
            Inner::Big(bx) => {
                if (*bx).is::<T>() {
                    Ok(Box::into_inner(bx.downcast::<T>().unwrap()))
                } else {
                    Err(Self(Inner::Big(bx)))
                }
            }
        }
    }
}
