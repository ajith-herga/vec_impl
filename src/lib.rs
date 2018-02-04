#![feature(allocator_api, ptr_internals, unique)]

use std::mem::{self, size_of};
use std::ptr::{self, Unique};
use std::heap::{Alloc, Heap, Layout};

pub struct MyVec<T> {
    my_vec: Unique<T>,
    layout: Layout,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            my_vec: Unique::empty(),
            layout: Layout::new::<()>(),
            len: 0,
        }
    }

    fn resize(&mut self) {
        // Allocate one size if len is 0
        if self.layout.size() == 0 {
            unsafe {
                let layout = Layout::array::<T>(32).unwrap();
                let ptr = Heap.alloc(layout.clone()).unwrap();
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        } else {
            // Reallocate if size is not zero.
            unsafe {
                let layout = Layout::array::<T>(self.layout.size() * 2).unwrap();
                let ptr = Heap.realloc(
                    mem::transmute(self.my_vec.as_ptr()),
                    self.layout.clone(),
                    layout.clone(),
                ).unwrap();
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
    }

    // TODO deallocate? drop will be called.
    pub fn push_back(&mut self, elem: T) {
        // if full, alloc
        if self.len * size_of::<T>() == self.layout.size() {
            self.resize();
        }
        // write
        // 1. find the offset, len?
        // TODO: self.len is usize, offset expects isize. Overflow?
        unsafe {
            ptr::write(self.my_vec.as_ptr().offset(self.len as isize), elem);
            self.len = self.len + 1;
        }
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe { self.my_vec.as_ptr().offset(index as isize).as_ref() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyVec;
    use std::mem::size_of;

    #[test]
    fn test_vec_i32() {
        let mut my_vec: MyVec<i32> = MyVec::new();
        my_vec.push_back(15);
        my_vec.push_back(0);
        my_vec.push_back(-150);
        assert_eq!(my_vec.layout.size(), 32 * size_of::<i32>());
        assert_eq!(my_vec.at(0), Some(&15));
        assert_eq!(my_vec.at(1), Some(&0));
        assert_eq!(my_vec.at(2), Some(&-150));
    }

    #[test]
    fn test_vec_rt() {
        struct RT {
            val: i32,
        }
        let mut my_vec: MyVec<RT> = MyVec::new();
        my_vec.push_back(RT { val: 15 });
        my_vec.push_back(RT { val: 0 });
        my_vec.push_back(RT { val: -150 });
        assert_eq!(my_vec.layout.size(), 32 * size_of::<RT>());
        assert_eq!(my_vec.at(0).is_some(), true);
        assert_eq!(my_vec.at(0).unwrap().val, 15);
    }
}
