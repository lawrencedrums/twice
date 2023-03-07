use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{Duration, Instant};

mod sorting;


fn main() {
    let size = 500;
    let sorted: Vec<i32> = (0..size).collect();
    let mut nums: Vec<i32> = (0..size).collect();

    let mut rng = thread_rng();
    nums.shuffle(&mut rng);

    let start = Instant::now();
    sorting::quicksort::quicksort(&mut nums);
    let duration = start.elapsed();

    assert_eq!(nums, sorted);
    println!("Sorting took {duration:?}.");
}