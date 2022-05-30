use std::boxed::Box;

struct Vector<T> {
    ptr: Box<T>,
    len: usize,
}