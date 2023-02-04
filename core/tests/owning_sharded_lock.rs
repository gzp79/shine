use crossbeam::sync::ShardedLock;
use shine_core::borrow::{OwningShardedReadGuard, OwningShardedWriteGuard};
use shine_test::test;
use std::sync::Arc;

struct Foo {
    value: String,
}

#[test]
fn owning_sharded_lock_ref_mut() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let mut guard = OwningShardedWriteGuard::new(Arc::new(ShardedLock::new(foo)));

    assert_eq!(guard.get().value, "bar");
    assert_eq!(guard.value, "bar");

    guard.get_mut().value = "bar2".to_string();
    assert_eq!(guard.get().value, "bar2");
    assert_eq!(guard.value, "bar2");
}

#[test]
fn owning_sharded_lock_ref() {
    let foo = Foo {
        value: "bar".to_string(),
    };
    let guard = OwningShardedReadGuard::new(Arc::new(ShardedLock::new(foo)));

    assert_eq!(guard.get().value, "bar");
    assert_eq!(guard.value, "bar");
}
