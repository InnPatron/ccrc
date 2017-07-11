use std::convert::AsRef;
use std::borrow::Borrow;
use std::cell::Cell;
use std::ops::{Drop, Deref};
use std::cmp;
use std::mem;
use std::fmt;

use super::collector;
use super::collectable::{Collectable, Tracer};

struct CcrcNode<T: Collectable> {
    value: T,
    data: NodeData,
}

pub struct NodeData {
    strong: Cell<usize>,
    weak: Cell<usize>,
    color: Cell<Color>,
    buffered: Cell<bool>,
}

impl<T: Collectable> Collectable for CcrcNode<T> {
    fn trace(&self, tracer: &Tracer) {
        Collectable::trace(&self.value, tracer);
    }
}

pub struct Ccrc<T: 'static + Collectable> {
    ptr: *mut CcrcNode<T>,      //NEEDS TO ALWAYS BE VALID
}

impl<T: Collectable> Ccrc<T> {
    pub fn new(value: T) -> Ccrc<T> {
        Ccrc {
            ptr: Box::into_raw(Box::new(CcrcNode {
                value: value,
                data: NodeData {
                    strong: Cell::new(1),
                    weak: Cell::new(1),
                    color: Cell::new(Color::Black),
                    buffered: Cell::new(false),
                }
            }))
        }
        
    }

    pub fn strong(&self) -> usize {
        self.inner().strong.get()
    }

    pub fn weak(&self) -> usize {
        self.inner().weak.get()
    }

    fn possible_root(&mut self) {
        if self.color() != Color::Purple {
            self.set_color(Color::Purple);
            if self.buffered() == false {
                self.set_buffered(true);
                let ptr: *mut CcrcNodePtr = self;
                collector::add_root(ptr);
            }
        }
    }

    pub fn ptr_eq(this: &Ccrc<T>, other: &Ccrc<T>) -> bool {
        this.ptr == other.ptr
    }

    pub fn downgrade(this: &Ccrc<T>) -> Weak<T> {
        this.inc_weak();
        Weak { ptr: this.ptr }
    }

    pub fn is_unique(this: &mut Ccrc<T>) -> bool {
        this.weak() == 0 && this.strong() == 1
    }

    pub fn get_mut(this: &mut Ccrc<T>) -> Option<&mut T> {
        if Ccrc::is_unique(this) {
            unsafe {
                Some(&mut this.ptr.as_mut().unwrap().value)
            }
        } else {
            None
        }
    }
}

impl<T: Collectable> Collectable for Ccrc<T> {
    fn trace(&self, tracer: &Tracer) {
        unsafe {
            Collectable::trace(&*self.ptr, tracer);
        }
    }
}

impl<T: Collectable> Clone for Ccrc<T> {
    fn clone(&self) -> Ccrc<T> {
        self.inc_strong();
        self.set_color(Color::Black);   //Increment
        Ccrc { ptr: self.ptr }
    }
}

impl<T: Collectable> Drop for Ccrc<T> {
    fn drop(&mut self) {
        if self.strong() > 0 {
            decrement(self);
        }
    }
}

fn decrement<T: Collectable>(n: &mut Ccrc<T>) {
    n.dec_strong();
    if n.strong() == 0 {
        release(n);
    } else {
        n.possible_root();
    }
}

fn release<T: Collectable>(n: &mut Ccrc<T>) {
    n.trace(&|child| child.dec_strong());
    n.set_color(Color::Black);
    if n.buffered() == false {
        n.free();
    }
}

impl<T: Collectable> Deref for Ccrc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            let data = &*self.ptr;
            &data.value
        }
    }
}

impl<T: Collectable> AsRef<T> for Ccrc<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: Collectable> Borrow<T> for Ccrc<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: Collectable + fmt::Debug> fmt::Debug for Ccrc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Collectable + fmt::Display> fmt::Display for Ccrc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Collectable + Ord> Ord for Ccrc<T> {
    fn cmp(&self, other: &Ccrc<T>) -> cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: Collectable + PartialOrd<T>> PartialOrd<Ccrc<T>> for Ccrc<T> {
    fn partial_cmp(&self, other: &Ccrc<T>) -> Option<cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Collectable + Eq> Eq for Ccrc<T> { }

impl<T: Collectable + PartialEq> PartialEq for Ccrc<T> {
    fn eq(&self, other: &Ccrc<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Collectable + Default> Default for Ccrc<T> {
    fn default() -> Ccrc<T> {
        Ccrc::new(Default::default())
    }
}

pub struct Weak<T: Collectable + 'static> {
    ptr: *mut CcrcNode<T>,
}

impl<T: Collectable> Weak<T> {
    pub fn upgrade(&self) -> Option<Ccrc<T>> {
        if self.strong() == 0 {
            None
        } else {
            self.inc_strong();
            Some(Ccrc { ptr: self.ptr })
        }
    }
}

impl<T: Collectable> Clone for Weak<T> {
    fn clone(&self) -> Weak<T> {
        self.inc_weak();
        Weak { ptr: self.ptr }
    }
}

impl<T: Collectable> Drop for Weak<T> {
    fn drop(&mut self) {
        self.free();
    }
}

impl<T: Collectable> Collectable for Weak<T> {
    fn trace(&self, tracer: &Tracer) {}
}

pub trait CcrcNodePtr: Collectable {

    fn free(&self);

    fn inner(&self) -> &NodeData;

    fn strong(&self) -> usize {
        self.inner().strong.get()
    }

    fn inc_strong(&self) {
        self.inner().strong.set(self.strong().checked_add(1).unwrap());
    }

    fn dec_strong(&self) {
        self.inner().strong.set(self.strong() - 1);     //TODO: add assertion self.strong > 0 before dec?
    }

    fn weak(&self) -> usize {
        self.inner().weak.get()
    }

    fn inc_weak(&self) {
        self.inner().weak.set(self.weak().checked_add(1).unwrap());
    }

    fn dec_weak(&self) {
        self.inner().weak.set(self.weak() - 1);     //TODO: add assertion self.weak > 0 before dec?
    }

    fn set_color(&self, color: Color) {
        self.inner().color.set(color);
    }

    fn color(&self) -> Color {
        self.inner().color.get()
    }

    fn buffered(&self) -> bool {
        self.inner().buffered.get()
    }

    fn set_buffered(&self, b: bool) {
        self.inner().buffered.set(b);
    }
}

impl<T: Collectable> CcrcNodePtr for Ccrc<T> {
    fn inner(&self) -> &NodeData {
        unsafe {
            &(*self.ptr).data
        }
    }

    fn free(&self) {
        unsafe {
            mem::drop(&(*self.ptr).value);
            self.dec_weak();
            if self.weak() == 0 {
                //TODO: deallocate backing node properly once no more pointers (weak or strong)
                mem::drop(Box::from_raw(self.ptr));
            }
        }
    }
}

impl<T: Collectable> CcrcNodePtr for Weak<T> {
    fn inner(&self) -> &NodeData {
        unsafe {
            &(*self.ptr).data
        }
    }

    fn free(&self) {
        unsafe {
            let ptr = self.ptr;
            self.dec_weak();
            if self.weak() == 0 {
                mem::drop(Box::from_raw(ptr))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,      // In use or free
    Gray,       // Possible member of cycle
    White,      // Member of garbage cycle
    Purple,     // Possible root of cycle
    Green,      // Acyclic
}
