use vec::MyVec;
use std::{fmt, vec::Vec};
pub struct MyMaxHeap<T> {
    my_heap: MyHeap<T>,
}

pub struct MyMinHeap<T> {
    my_heap: MyHeap<T>,
}

pub struct MyHeap<T> {
    my_vec: MyVec<T>,
    max: bool,
}

impl<T: Ord> MyHeap<T> {
    pub fn new(max: bool) -> Self {
        MyHeap {
            my_vec: MyVec::new(None),
            max: max,
        }
    }

    pub fn add(&mut self, elem: T) {
        self.my_vec.push_back(elem);

        let mut child = self.my_vec.len() - 1;

        while child != 0 {
            let parent = (child - 1)/ 2;
            let parent_smaller = self.my_vec[child] > self.my_vec[parent];
            let child_smaller = !parent_smaller;
            if self.max && parent_smaller || !self.max && child_smaller
            {
                self.my_vec.swap(parent, child);
            }
            child = parent;
        }
        return;
    }

    pub fn top(&self) -> Option<&T> {
        self.my_vec.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.my_vec.len() == 0 {
            return self.my_vec.pop();
        }

        let mut last = self.my_vec.len() - 1;
        self.my_vec.swap(last, 0);
        let top = self.my_vec.pop();

        if last == 0 {
            return top;
        }

        last = self.my_vec.len() - 1;
        let mut parent = 0;
        // If both children > len stop.
        while parent*2 < last {
            // Find the loudest among children
            let child1 = parent*2 + 1;
            let child2 = child1 + 1;
            let mut loudest = child1;
            if child2 <= last {
                if self.max {
                    loudest = if self.my_vec[child1] > self.my_vec[child2] {child1}
                        else {child2}
                } else {
                    loudest = if self.my_vec[child1] < self.my_vec[child2] {child1}
                        else {child2}
                }

            }
            /*
             * Check if the heap property is met. If so, return early, no need to check the rest.
             */
            if self.my_vec[parent] == self.my_vec[loudest] ||
                self.max && self.my_vec[parent] > self.my_vec[loudest] ||
                !self.max && self.my_vec[parent] < self.my_vec[loudest]
            {
                break;
            }
            // Swap to return heap property at this level
            self.my_vec.swap(parent, loudest);
            /*
             * Move down to loudest child root as swap may have broken heap property there.
             */
            parent = loudest;
        }
        return top;
    }

    pub fn to_vec(mut self) -> Vec<T> {
        let mut list : Vec<T> = Vec::new();
        while let Some(elem) = self.pop() {
            list.push(elem);
        }
        return list;
    }


    pub fn from_vec(elems: Vec<T>, max: bool) -> Self {
        let mut heap: MyHeap<T> = MyHeap::new(max);
        for elem in elems.into_iter() {
            heap.add(elem);
        }
        return heap
    }
}

impl<T: fmt::Display> fmt::Display for MyHeap<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(formatter, ", {}", self.my_vec)
    }
}

impl<T: Ord> MyMinHeap<T> {
    pub fn new() -> Self {
        MyMinHeap {
            my_heap: MyHeap::new(false),
        }
    }

    pub fn add(&mut self, elem: T) {
    }

    pub fn top(&self) -> Option<&T> {
        self.my_heap.top()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.my_heap.pop()
    }

    pub fn from_vec(elems: Vec<T>)  -> Self {
        let mut heap: MyHeap<T> = MyHeap::new(false);
        for elem in elems.into_iter() {
            heap.add(elem);
        }
        MyMinHeap {
            my_heap: heap,
        }
    }

    pub fn to_vec(mut self) -> Vec<T> {
        self.my_heap.to_vec()
    }
}

impl<T: Ord> MyMaxHeap<T> {
    pub fn new() -> Self {
        MyMaxHeap {
            my_heap: MyHeap::new(true),
        }
    }

    pub fn add(&mut self, elem: T) {
    }

    pub fn top(&self) -> Option<&T> {
        self.my_heap.top()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.my_heap.pop()
    }

    pub fn from_vec(elems: Vec<T>)  -> Self {
        let mut heap: MyHeap<T> = MyHeap::new(true);
        for elem in elems.into_iter() {
            heap.add(elem);
        }
        MyMaxHeap {
            my_heap: heap,
        }
    }

    pub fn to_vec(mut self) -> Vec<T> {
        self.my_heap.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::MyMaxHeap;
    use super::MyMinHeap;
    #[test]
    fn test_none() {
        let mut heap: MyMaxHeap<i32> = MyMaxHeap::new();
        assert_eq!(heap.top(), None);
        assert_eq!(heap.pop(), None);
        let mut heap: MyMinHeap<i32> = MyMinHeap::new();
        assert_eq!(heap.top(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_sort() {
        let sorted_vec = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let rev_sorted_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let heap = MyMaxHeap::from_vec(rev_sorted_vec.clone());
        assert_eq!(heap.to_vec(), sorted_vec);
        let heap = MyMinHeap::from_vec(sorted_vec);
        assert_eq!(heap.to_vec(), rev_sorted_vec);
    }
}
