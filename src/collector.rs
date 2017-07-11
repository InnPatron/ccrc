use std::cell::RefCell;
use ccrc::{ Color, CcrcNodePtr };

thread_local! {
    static ROOT_BUFF: RefCell<Vec<*mut CcrcNodePtr>> = RefCell::new(vec![]);
}

// SHOULD ONLY BE FED BY 'Ccrc' FOR NOW 
pub fn add_root(ptr: *mut CcrcNodePtr) {
    ROOT_BUFF.with(|refcell| {
        refcell.borrow_mut().push(ptr);
    })
}

pub fn collect_cycles() {
    mark_roots();
    scan_roots();
    collect_roots();
}

fn mark_roots() {
    let old_buff: Vec<_> = ROOT_BUFF.with (|refcell| {
        let mut borrow = refcell.borrow_mut();
        let drained = borrow.drain(..);
        drained.collect()
    });

    for raw_root in old_buff.iter() {
        unsafe {
            let root: &CcrcNodePtr = &**raw_root;
            if root.color() == Color::Purple {
                mark_gray(root);
                ROOT_BUFF.with(|refcell| {
                    refcell.borrow_mut().push(raw_root.clone());
                });
            } else {
                root.set_buffered(false);
                if root.color() == Color::Black && root.strong() == 0 {
                    //no more strong refs -> strong refs lose implicit weak ref
                    root.dec_weak();
                    root.free();
                }
            }
        }
    }
}

fn mark_gray(node: &CcrcNodePtr) {
    if node.color() != Color::Gray {
        node.set_color(Color::Gray);
        node.trace(&|child| {
            child.dec_strong();
            mark_gray(child);
        });
    }
}

fn scan_roots() {
    ROOT_BUFF.with(|refcell| {
        for root in refcell.borrow().iter() {
            unsafe {
                scan(&**root);
            }
        }
    });
}

fn scan(node: &CcrcNodePtr) {
    if node.color() == Color::Gray {
        if node.strong() > 0 {
            scan_black(node);
        } else {
            node.set_color(Color::White);
            node.trace(&|child| scan(child));
        }
    }
}

fn scan_black(node: &CcrcNodePtr) {
    node.set_color(Color::Black);
    node.trace(&|child| {
        child.inc_strong();
        if child.color() != Color::Black {
            scan_black(child);
        }
    });
}

fn collect_roots() {
    let old_buff: Vec<_> = ROOT_BUFF.with (|refcell| {
        let mut borrow = refcell.borrow_mut();
        let drained = borrow.drain(..);
        drained.collect()
    });
    //TODO: probably not the best way to do this...

    for root in old_buff.iter() {
        unsafe {
            let root = &**root;
            root.set_buffered(false);
            collect_white(root);
        }
    }
}

fn collect_white(node: &CcrcNodePtr) {
    if node.color() == Color::White && node.buffered() == false {
        node.set_color(Color::Black);
        node.trace(&|child| collect_white(child));
        node.free();
    }
}
