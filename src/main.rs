#![allow(unused)]

use std::vec;
use rand::{thread_rng, Rng};
use rand::distributions::Standard;

fn merge(left: Vec<u64>, right: Vec<u64>) -> Vec<u64> {
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

fn merge_sort(vec: Vec<u64>) -> Vec<u64> {
    if vec.len() <= 1 {
        return vec;
    }

    let mid = vec.len() / 2;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 0..mid {
        left.push(vec[i]);
    }

    for i in mid..vec.len() {
        right.push(vec[i]);
    }

    left = merge_sort(left);
    right = merge_sort(right);

    merge(left, right)

}

mod sorting {
    use rand::Rng;


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

    pub fn merge_sort(vec: Vec<u64>) -> Vec<u64> {
        if vec.len() <= 1 {
            return vec;
        }

        let mid = vec.len() / 2;
        let mut left = Vec::new();
        let mut right = Vec::new();

        for i in 0..mid {
            left.push(vec[i]);
        }

        for i in mid..vec.len() {
            right.push(vec[i]);
        }

        left = merge_sort(left);
        right = merge_sort(right);

        merge(left, right)
    }

    pub fn generate_random_array(size: usize) -> Vec<u64> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen()).collect()
    }
}

fn main() {
    let my_array = sorting::generate_random_array(8);
    println!("Unsorted array: {:?}", my_array);
    let sorted_array = sorting::merge_sort(my_array);
    println!("Sorted array: {:?}", sorted_array);
}
