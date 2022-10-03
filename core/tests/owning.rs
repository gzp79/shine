use shine_core::{atomic_refcell::AtomicRefCell, borrow::OwningAtomicRefMutGuard};
use shine_test::test;
use std::sync::Arc;

#[test]
#[ignore]
fn check_compilation() {
    let mut guard = OwningAtomicRefMutGuard::new(Arc::new(AtomicRefCell::new(1u8)));

    let _a = guard.get();
    let _b = guard.get_mut();
}
