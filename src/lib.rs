#![feature(allocator_api, ptr_internals, unique)]

use std::mem::{self, size_of};
use std::ptr::{self, Unique};
use std::heap::{Alloc, AllocErr, Heap, Layout};

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

    fn resize(&mut self) -> Result<(), AllocErr> {
        // Allocate one size if len is 0
        if self.layout.size() == 0 {
            unsafe {
                let layout = Layout::array::<T>(32).unwrap();
                let ptr = Heap.alloc(layout.clone())?;
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
                )?;
                self.layout = layout;
                self.my_vec = Unique::new_unchecked(mem::transmute(ptr));
            }
        }
        Ok(())
    }

    // TODO deallocate? drop will be called.
    pub fn push_back(&mut self, elem: T) -> Result<(), AllocErr> {
        // if full, alloc
        if self.len * size_of::<T>() == self.layout.size() {
            self.resize()?;
        }
        // write
        // 1. find the offset, len?
        // TODO: self.len is usize, offset expects isize. Overflow?
        unsafe {
            ptr::write(self.my_vec.as_ptr().offset(self.len as isize), elem);
            self.len = self.len + 1;
        }
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
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
        my_vec.push_back(15).unwrap();
        my_vec.push_back(0).unwrap();
        my_vec.push_back(-150).unwrap();
        assert_eq!(my_vec.layout.size(), 32 * size_of::<i32>());
        assert_eq!(my_vec.get(0), Some(&15));
        assert_eq!(my_vec.get(1), Some(&0));
        assert_eq!(my_vec.get(2), Some(&-150));
    }

    #[test]
    fn test_vec_int() {
        let ints = vec![1, 2, 3, 4, 5];
        let mut my_vec: MyVec<i32> = MyVec::new();

        for elem in ints.iter() {
            my_vec.push_back(*elem).unwrap();
        }

        assert_eq!(my_vec.layout.size(), 32 * size_of::<i32>());

        for i in 0..30 {
            assert_eq!(my_vec.get(i), ints.get(i));
        }
    }

    #[test]
    fn test_vec_str() {
        let strings = vec!["So", "far", "so", "good"];
        let mut my_vec: MyVec<&str> = MyVec::new();

        for elem in strings.iter() {
            my_vec.push_back(elem).unwrap();
        }

        assert_eq!(my_vec.layout.size(), 32 * size_of::<&str>());

        for i in 0..30 {
            assert_eq!(my_vec.get(i), strings.get(i));
        }
    }

    #[test]
    fn test_vec_rt_ref() {
        #[derive(Debug)]
        struct RT<'a> {
            val: i32,
            name: &'a str,
        }

        impl<'a> RT<'a> {
            fn new(val: i32) -> Self {
                RT { val, name: "Ajith" }
            }
        }

        let rts = vec![
            RT::new(15),
            RT::new(150),
            RT::new(0),
            RT::new(-1),
            RT::new(150),
        ];
        let mut my_vec: MyVec<&RT> = MyVec::new();

        for elem in rts.iter() {
            my_vec.push_back(elem).unwrap();
        }

        assert_eq!(my_vec.layout.size(), 32 * size_of::<&RT>());

        for i in 0..rts.len() {
            assert_eq!(my_vec.get(i).unwrap().val, rts.get(i).unwrap().val);
        }
    }
}
