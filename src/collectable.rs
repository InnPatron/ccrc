use super::ccrc::CcrcNodePtr;

pub type Tracer = Fn(&CcrcNodePtr);

pub trait Collectable {
    fn trace(&self, tracer: &Tracer);
}

impl Collectable for bool {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for char {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for i8 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for i16 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for i32 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for i64 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for u8 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for u16 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for u32 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for u64 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for isize {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for usize {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for f32 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for f64 {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for String {
    fn trace(&self, tracer: &Tracer) {
     
    }
}

impl Collectable for str {
    fn trace(&self, tracer: &Tracer) {
     
    }
}
