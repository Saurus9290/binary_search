use core::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: Ord>(space:R,key:T) -> Option<usize>{
    let space = space.as_ref();
    let mid = space.len() / 2;
    match key.cmp(space.get(mid)?){
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&space[..mid],key),
        Ordering::Greater => find(&space[mid+1..],key).map(|i| i + mid + 1),
    }
} 

fn main() {
    let sorted_data = vec![1,3,5,7,9,11,13,15,17,19,21];
    let key_to_find = 7;

    match find(&sorted_data, key_to_find) {
        Some(index) => println!("Found key {} at index {}", key_to_find, index),
        None => println!("Key {} not found", key_to_find),
    }
}