use shine_core::collections::{small_box_layout, SmallAnyBox};
use shine_test::test;
use std::{cell::RefCell, rc::Rc};

#[test]
fn simple() {
    let mut bx = SmallAnyBox::<small_box_layout::S64_8>::new(13u16);
    assert!(bx.is_small());
    assert_eq!(bx.as_ref::<u8>(), None);
    assert_eq!(bx.as_mut::<u8>(), None);
    assert_eq!(bx.as_ref::<u16>(), Some(&13));
    bx.as_mut::<u16>().map(|b| *b = 14);
    let bx = bx.take_as::<u8>().err().unwrap();
    assert_eq!(bx.take_as::<u16>().ok().unwrap(), 14);

    let mut bx = SmallAnyBox::<small_box_layout::S8_1>::new(13u16);
    assert!(bx.is_big());
    assert_eq!(bx.as_ref::<u8>(), None);
    assert_eq!(bx.as_ref::<u16>(), Some(&13));
    assert_eq!(bx.as_mut::<u8>(), None);
    bx.as_mut::<u16>().map(|b| *b = 14);
    let bx = bx.take_as::<u8>().err().unwrap();
    assert_eq!(bx.take_as::<u16>().ok().unwrap(), 14);

    let mut bx = SmallAnyBox::<small_box_layout::S64_8>::new("string".to_string());
    assert!(bx.is_small());
    assert_eq!(bx.as_ref::<u8>(), None);
    assert_eq!(bx.as_mut::<u8>(), None);
    assert_eq!(bx.as_ref::<String>().unwrap(), "string");
    bx.as_mut::<String>().map(|b| *b = "string2".to_string());
    assert_eq!(bx.take_as::<String>().ok().unwrap(), "string2");

    let mut bx = SmallAnyBox::<small_box_layout::S8_1>::new("string".to_string());
    assert!(bx.is_big());
    assert_eq!(bx.as_ref::<u8>(), None);
    assert_eq!(bx.as_mut::<u8>(), None);
    assert_eq!(bx.as_ref::<String>().unwrap(), "string");
    bx.as_mut::<String>().map(|b| *b = "string2".to_string());
    assert_eq!(bx.take_as::<String>().ok().unwrap(), "string2");
}

#[derive(Debug)]
struct TrackDrop(Rc<RefCell<usize>>);

impl Drop for TrackDrop {
    fn drop(&mut self) {
        *self.0.borrow_mut() += 1;
    }
}

#[test]
fn drop() {
    let count = Rc::new(RefCell::new(0));
    {
        let _bx = TrackDrop(count.clone());
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S64_8>::new(TrackDrop(count.clone()));
        assert!(bx.is_small());
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S64_8>::new(TrackDrop(count.clone()));
        assert!(bx.is_small());
        let _ = bx.take_as::<TrackDrop>().ok().unwrap();
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S64_8>::new(TrackDrop(count.clone()));
        assert!(bx.is_small());
        let _ = bx.take_as::<String>().err().unwrap();
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S8_1>::new(TrackDrop(count.clone()));
        assert!(bx.is_big());
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S8_1>::new(TrackDrop(count.clone()));
        assert!(bx.is_big());
        let _ = bx.take_as::<TrackDrop>().ok().unwrap();
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);

    let count = Rc::new(RefCell::new(0));
    {
        let bx = SmallAnyBox::<small_box_layout::S8_1>::new(TrackDrop(count.clone()));
        assert!(bx.is_big());
        let _ = bx.take_as::<String>().err().unwrap();
        let cnt = *count.borrow();
        cnt
    };
    assert_eq!(*count.borrow(), 1);
}
