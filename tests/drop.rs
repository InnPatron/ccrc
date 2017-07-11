extern crate ccrc;
use ccrc::*;
use std::ops::Drop;
use std::cell::RefCell;

#[test]
fn strong_only() {
    let a = Ccrc::new(51);
    assert_eq!(a.strong(), 1);
}

#[test]
fn strong_and_weak() {
    let a = Ccrc::new(51);
    assert_eq!(a.strong(), 1);
    let b = Ccrc::downgrade(&a);
    let c = Ccrc::downgrade(&a);
    assert_eq!(a.weak(), 3);
}

struct Test(RefCell<Option<Ccrc<Test>>>);

impl Collectable for Test {
    fn trace(&self, tracer: &Tracer) {
        match *self.0.borrow() {
            Some(ref child) => tracer(child),
            None => ()
        }
    }
}

#[test]
fn strong_cycle() {
    let a = Ccrc::new(Test(RefCell::new(None)));
    let b = Ccrc::new(Test(RefCell::new(Some(a.clone()))));
    *a.0.borrow_mut() = Some(b.clone());
    ccrc::collect_cycles();
}

#[test]
#[should_panic]
fn invalid_upgrade() {
    let weak = {
        let a = Ccrc::new(Test(RefCell::new(None)));
        let b = Ccrc::new(Test(RefCell::new(Some(a.clone()))));
        Ccrc::downgrade(&a)
    };
    weak.upgrade().expect("FAIL");
}

#[test]
#[should_panic]
fn invalid_upgrade_chained() {
    let weak = {
        let a = Ccrc::new(Test(RefCell::new(None)));
        let b = Ccrc::new(Test(RefCell::new(Some(a.clone()))));
        Ccrc::downgrade(&b)
    };
    weak.upgrade().expect("FAIL");
}

#[test]
#[should_panic]
fn invalid_upgrade_cycled() {
    let weak_a = {
        let a = Ccrc::new(Test(RefCell::new(None)));
        let b = Ccrc::new(Test(RefCell::new(Some(a.clone()))));
        *a.0.borrow_mut() = Some(b.clone());
        Ccrc::downgrade(&a)
    };
    //a && b have gone out of scope, leaving behind 
    ccrc::collect_cycles();
    {
        weak_a.upgrade().unwrap();
    }
}

#[test]
#[should_panic]
fn invalid_upgrade_cycled_chain() {
    let weak_b = {
        let a = Ccrc::new(Test(RefCell::new(None)));
        let b = Ccrc::new(Test(RefCell::new(Some(a.clone()))));
        *a.0.borrow_mut() = Some(b.clone());
        Ccrc::downgrade(&b)
    };
    //a && b have gone out of scope, leaving behind 
    ccrc::collect_cycles();
    {
        weak_b.upgrade().unwrap();
    }
}
