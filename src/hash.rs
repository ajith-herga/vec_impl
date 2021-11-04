use vec::MyVec;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

enum HashElement<T> {
    Empty,
    Deleted,
    Valid(T),
}

pub struct MyHashSet1<T> {
    my_vec: MyVec<HashElement<T>>,
}

impl<T> MyHashSet1<T> where T: PartialEq, T: Eq, T: Hash {
    pub fn new() -> Self {
        let mut vec: MyVec<HashElement<T>>  = MyVec::new(Some(10000));
        for _ in 0..10000 {
            vec.push_back(HashElement::Empty);
        }
        MyHashSet1 {
            my_vec : vec,
        }
    }
    fn myHashGetProbes(&self, key: &T) -> MyVec<usize> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let slot = hasher.finish() % self.my_vec.len() as u64;
        let mut ret = MyVec::new(Some(10));
        for probe in 0..20 {
            ret.push_back(((slot + probe) % self.my_vec.len() as u64) as usize);
        }
        return ret;
    }

    pub fn contains(&self, key: &T) ->  bool {
        let mut found = false;
        for probe in self.myHashGetProbes(key).into_iter() {
            let val = self.my_vec.get(probe).unwrap();
            match val {
                HashElement::Valid(rval) => {
                    if rval == key {
                        found = true;
                        break;
                    }
                }
                HashElement::Empty => break,
                HashElement::Deleted => continue,
            }
        }
        return found;
    }

    pub fn remove(&mut self, key: &T) {
        for probe in self.myHashGetProbes(key).into_iter() {
            let val = self.my_vec.get_mut(probe).unwrap();
            match val {
                HashElement::Valid(rval) => {
                    if rval == key {
                        *val = HashElement::Deleted; 
                        break;
                    }
                }
                HashElement::Empty => break,
                HashElement::Deleted => continue,
            }
        }
    }

    pub fn add(&mut self, key: T) {
        if self.contains(&key) {
            return;
        }
        for probe in self.myHashGetProbes(&key).into_iter() {
            let val = self.my_vec.get_mut(probe).unwrap();
            if let HashElement::Valid(_) = val {
                continue;
            }
            *val = HashElement::Valid(key); 
            break;
        }
    }
}
