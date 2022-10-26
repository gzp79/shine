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
pub mod graph;

pub use atomic_refcell;
pub use crossbeam;
pub use downcast_rs;
pub use slotmap;
pub use smallbox;

/// Helper to extend lifetime of a refernece. Genrates highly unsafe code.
#[macro_export]
macro_rules! extend_lifetime {
    ($d:ident) => {
        &*($d as *const _)
    };
}

/// Helper to construct hasmap from key => value lists
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

/// Helper to construct hasset from value lists
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

/// Helper to construct hasmap from key => value lists
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

/// Helper to construct hasset from value lists
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
