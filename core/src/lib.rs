#![feature(entry_insert)]
#![feature(box_into_inner)]

mod base64url;
pub use self::base64url::*;
mod simple_error;
pub use self::simple_error::*;
mod small_string_id;
pub use self::small_string_id::*;
mod delay;
pub use self::delay::*;

pub mod borrow;
pub mod collections;

pub use atomic_refcell;
pub use crossbeam;
pub use downcast_rs;
pub use slotmap;

pub mod smallbox {
    pub use ::smallbox::*;

    /// Space constraint for SmallBux using const generics.
    #[cfg(not(miri))]
    pub struct Space<const N: usize> {
        _dummy: [usize; N]
    }

    /// Space constraint for SmallBox under miri. 
    /// SmallBox has some issue with miri: https://github.com/andylokandy/smallbox/issues/21#issuecomment-1418204906, thus we fall back to the
    /// heap storage.
    #[cfg(miri)]
    pub struct Space<const N: usize> {
        _phantomData: std::marker::PhantomData<[usize; N]>
    }

}
/// Helper to construct a HashMap from a list of `key => value` pairs.
#[macro_export]
macro_rules! hash_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }};
    ($( $key: expr => $val: expr, )*) => {
        hash_map!{$( $key => $val ),*}
    };
}

/// Helper to construct a HashSet from a list of values
#[macro_export]
macro_rules! hash_set {
    ($( $val: expr ),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($val); )*
         set
    }};
    ($( $val: expr, )*) => {
        hash_set!{$( $val ),*}
    };
}

/// Helper to construct a BTreemMap from a list of `key => value` pairs.
#[macro_export]
macro_rules! btree_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::BTreeMap::new();
         $( map.insert($key, $val); )*
         map
    }};
    ($( $key: expr => $val: expr, )*) => {
        btree_map!{$( $key => $val ),*}
    };
}

/// Helper to construct BTreeSet from a list of values
#[macro_export]
macro_rules! btree_set {
    ($( $val: expr ),*) => {{
         let mut set = ::std::collections::BTreeSet::new();
         $( set.insert($val); )*
         set
    }};
    ($( $val: expr, )*) => {
        btree_set!{$( $val ),*}
    };
}
