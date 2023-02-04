use shine_core::{
    atomic_refcell::AtomicRefCell,
    borrow::{OwningAtomicRefGuard, OwningAtomicRefMutGuard},
};
use shine_test::test;
use std::sync::Arc;

struct Foo {
    value: String,
}

#[test]
fn owning_atomic_ref_mut() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let mut guard = OwningAtomicRefMutGuard::new(Arc::new(AtomicRefCell::new(foo)));

    assert_eq!(guard.get().value, "bar");
    assert_eq!(guard.value, "bar");

    guard.get_mut().value = "bar2".to_string();
    assert_eq!(guard.get().value, "bar2");
    assert_eq!(guard.value, "bar2");
}

#[test]
fn owning_atomic_ref_mut_map() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefMutGuard::new(Arc::new(AtomicRefCell::new(foo)));

    let mut guard = guard.map(|x| &mut x.value);
    *guard.get_mut() = "bar2".to_string();
    assert_eq!(guard.get(), "bar2");
    assert_eq!(&*guard, "bar2");
}

#[test]
fn owning_atomic_ref_mut_try_map_ok() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefMutGuard::new(Arc::new(AtomicRefCell::new(foo)));

    let mut guard = guard
        .try_map(|x| if x.value.len() == 3 { Ok(&mut x.value) } else { Err(1) })
        .unwrap();
    *guard.get_mut() = "bar2".to_string();
    assert_eq!(guard.get(), "bar2");
    assert_eq!(&*guard, "bar2");
}

#[test]
fn owning_atomic_ref_mut_try_map_err() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefMutGuard::new(Arc::new(AtomicRefCell::new(foo)));

    let guard = guard
        .try_map(|x| if x.value.len() == 1 { Ok(&mut x.value) } else { Err(1) })
        .err()
        .unwrap();
    assert_eq!(guard, 1);
}

#[test]
fn owning_atomic_ref() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefGuard::new(Arc::new(AtomicRefCell::new(foo)));

    assert_eq!(guard.get().value, "bar");
    assert_eq!(guard.value, "bar");
}

#[test]
fn owning_atomic_ref_map() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefGuard::new(Arc::new(AtomicRefCell::new(foo)));

    assert_eq!(guard.get().value, "bar");

    let guard = guard.map(|x| &x.value);
    assert_eq!(guard.get(), "bar");
    assert_eq!(&*guard, "bar");
}
#[test]
fn owning_atomic_ref_try_map_ok() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefGuard::new(Arc::new(AtomicRefCell::new(foo)));

    let guard = guard
        .try_map(|x| if x.value.len() == 3 { Ok(&x.value) } else { Err(1) })
        .unwrap();
    assert_eq!(guard.get(), "bar");
    assert_eq!(&*guard, "bar");
}

#[test]
fn owning_atomic_ref_try_map_err() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningAtomicRefGuard::new(Arc::new(AtomicRefCell::new(foo)));

    let guard = guard
        .try_map(|x| if x.value.len() == 1 { Ok(&x.value) } else { Err(1) })
        .err()
        .unwrap();
    assert_eq!(guard, 1);
}
