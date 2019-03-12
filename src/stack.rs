use vec::MyVec;

pub struct MyStack<T> {
    my_vec: MyVec<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack {
            my_vec: MyVec::new(None),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.my_vec.push_back(elem);
    }

    pub fn top(&self) -> Option<&T> {
        self.my_vec.back()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.my_vec.pop()
    }
}
