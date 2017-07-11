use std::rc::Rc;
use std::cell::Cell;
use std::ops::Drop;

pub struct Ccrc<T> where T: ?Sized {
    ptr: Rc<T>,
    color: Cell<Color>,
    buffered: bool,
}

impl<T> Ccrc<T> {
    pub fn new(value: T) -> Ccrc<T> {
        Ccrc {
            ptr: Rc::new(value),
            color: Cell::new(Color::Black),
            buffered: false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Black,      // In use or free
    Gray,       // Possible member of cycle
    White,      // Member of garbage cycle
    Purple,     // Possible root of cycle
    Green,      // Acyclic
}

impl<T> Clone for Ccrc<T> where T: ?Sized {
    fn clone(&self) -> Ccrc<T> {
        Ccrc {
            ptr: self.ptr.clone(),
            color: Cell::new(Color::Black),
            buffered: false,
        }
    }
}

impl<T> Drop for Ccrc<T> where T: ?Sized {
    fn drop(&mut self) {
        let strong_count = Rc::strong_count(&self.ptr);
        if strong_count > 0 {    //TODO: Should this be '> 1' b/c strong_count hasn't been deprecated yet
            self.color.set(Color::Purple);  // Possible root of cycle
            if buffered == false {
                //TODO: Add self to buffer of possible roots
            }
        } else if strong_count == 0 {    //TODO: Should this be '> 1' b/c strong_count hasn't been deprecated yet
            self.color.set(Color::Black);   // Underlying object is not droppable
        }
    }
}
