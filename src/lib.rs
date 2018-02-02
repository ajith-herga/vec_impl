#![feature(allocator_api, ptr_internals, unique)]

use std::mem;
use std::ptr::{Unique};
use std::heap::{Alloc, Heap, Layout};

pub struct Vec<T> {
vec: Unique<T>,
layout: Layout,
len: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec { vec: Unique::empty(), layout: Layout::new::<()>(), len: 0 }
    }

    // TODO deallocate? drop will be called.
    fn insert(&mut self, elem: T) {
        // if full, alloc
        if self.len == self.layout.size() {
            // Allocate one size if len is 0
            if self.layout.size() == 0 {
                unsafe {
                    let layout = Layout::array::<T>(32).unwrap();
                    let ptr = Heap.alloc(layout.clone()).unwrap();
                    self.layout = layout;
                    self.vec = Unique::new_unchecked(mem::transmute(ptr));
                }
            } else {
            // Reallocate if size is not zero.
                unsafe {
                    let layout = Layout::array::<T>(self.layout.size()*2).unwrap();
                    let ptr = Heap.realloc(
                        mem::transmute(self.vec.as_ptr()),
                        self.layout.clone(),
                        layout.clone()
                        ).unwrap();
                    self.layout = layout;
                    self.vec = Unique::new_unchecked(mem::transmute(ptr));
                }
            }
        }
        // write
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let vec = Vec::new();

        assert_eq!(2 + 2, 4);
    }
}
