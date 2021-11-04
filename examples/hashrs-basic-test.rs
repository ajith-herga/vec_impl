/* Simple hash table */
extern crate vec_impl;

fn main() {
    let mut hset = vec_impl::MyHashSet1::new();
    for ind in 0..10 {
        let key = ind * 10000;
        println!("Has key {} ? {}", key, hset.contains(&key));
        hset.add(key);
        println!("Has key {} ? {}", key, hset.contains(&key));
    }
    for ind in 0..10 {
        let key = ind * 10000;
        println!("Has key {} ? {}", key, hset.contains(&key));
        hset.remove(&key);
        println!("Has key {} ? {}", key, hset.contains(&key));
    }
}
