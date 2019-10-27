use vec::MyVec;
use std::vec::Vec;

pub struct MyHeap<T: Ord> {
    my_vec: MyVec<T>,
}

impl<T: Ord> MyHeap<T> {
    pub fn new() -> Self {
        MyHeap {
            my_vec: MyVec::new(None),
        }
    }

    pub fn add(&mut self, elem: T) {
        self.my_vec.push_back(elem);

        let mut child = self.my_vec.len() - 1;

        while child != 0 {
            let parent = (child - 1)/ 2;
            if self.my_vec[child] > self.my_vec[parent] {
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
            let child1 = parent*2 + 1;
            let child2 = child1 + 1;
            let mut greatest = child1;
            if child2 <= last {
                greatest = if self.my_vec[child1] > self.my_vec[child2] {child1}
                    else {child2}
            }
            self.my_vec.swap(parent, greatest);
            parent = greatest;
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

    pub fn from_vec(elems: Vec<T>) -> Self {
        let mut heap: MyHeap<T> = MyHeap::new();
        for elem in elems.into_iter() {
            heap.add(elem);
        }
        return heap
    }
}

#[cfg(test)]
mod tests {
    use super::MyHeap;
    #[test]
    fn test_none() {
        let mut heap: MyHeap<i32> = MyHeap::new();
        assert_eq!(heap.top(), None);
        assert_eq!(heap.pop(), None);
    }
    /*
    #[test]
    fn test_heap_sort() {
        let sorted_vec = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let unsorted_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let heap = MyHeap::from_vec(unsorted_vec);
        assert_eq!(heap.to_vec(), sorted_vec);
    }
    */
    #[test]
    fn test_heap_add() {
        let unsorted_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut heap = MyHeap::new();
        for elem in unsorted_vec.iter() {
            heap.add(elem);
        }
        while let Some(elem) = heap.pop() {
            println!("Top: {}", elem);
        }
        assert_eq!(heap.to_vec().len(), 1);
    }
}
