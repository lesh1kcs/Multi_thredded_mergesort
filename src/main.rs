mod sorting {
    use rand::Rng;
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn merge(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
        let mut result = Vec::new();
        let mut left_iter = left.into_iter();
        let mut right_iter = right.into_iter();

        let mut left_next = left_iter.next();
        let mut right_next = right_iter.next();

        loop {
            match (left_next, right_next) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        result.push(l);
                        left_next = left_iter.next();
                    } else {
                        result.push(r);
                        right_next = right_iter.next();
                    }
                }
                (Some(l), None) => {
                    result.push(l);
                    result.extend(left_iter);
                    break;
                }
                (None, Some(r)) => {
                    result.push(r);
                    result.extend(right_iter);
                    break;
                }
                (None, None) => break,
            }
        }

        result
    }

    pub fn merge_sort(vec: Vec<u64>, max_depth: usize, current_depth: usize) -> Vec<u64> {
        if vec.len() <= 1 {
            return vec;
        }

        let mid = vec.len() / 2;
        let left = vec[..mid].to_vec();
        let right = vec[mid..].to_vec();

        if current_depth >= max_depth {
            // If the current depth is greater than or equal to max_depth, sort sequentially
            let left_sorted = merge_sort(left, max_depth, current_depth + 1);
            let right_sorted = merge_sort(right, max_depth, current_depth + 1);
            return merge(left_sorted, right_sorted);
        }

        let left_sorted = Arc::new(Mutex::new(Vec::new()));
        let right_sorted = Arc::new(Mutex::new(Vec::new()));

        let left_sorted_clone = Arc::clone(&left_sorted);
        let right_sorted_clone = Arc::clone(&right_sorted);

        let left_handle = thread::spawn(move || {
            let sorted = merge_sort(left, max_depth, current_depth + 1);
            let mut left_sorted = left_sorted_clone.lock().unwrap();
            *left_sorted = sorted;
        });

        let right_handle = thread::spawn(move || {
            let sorted = merge_sort(right, max_depth, current_depth + 1);
            let mut right_sorted = right_sorted_clone.lock().unwrap();
            *right_sorted = sorted;
        });

        left_handle.join().unwrap();
        right_handle.join().unwrap();

        let left_sorted = Arc::try_unwrap(left_sorted).unwrap().into_inner().unwrap();
        let right_sorted = Arc::try_unwrap(right_sorted).unwrap().into_inner().unwrap();

        merge(left_sorted, right_sorted)
    }

    pub fn generate_random_array(size: usize) -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..1000000)).collect()
    }
}

fn main() {
    let my_array = sorting::generate_random_array(80);
    println!("Unsorted array: {:?}", my_array);
    let max_depth = 4; // Set the maximum depth for creating new threads (2^4 = 16 threads)
    let sorted_array = sorting::merge_sort(my_array, max_depth, 0);
    println!("Sorted array: {:?}", sorted_array);
}