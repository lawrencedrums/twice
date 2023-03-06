use rand::seq::SliceRandom;
use rand::thread_rng;

mod sorting;


fn main() {
    let size = 200;

    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (0..size).collect();
    let sorted: Vec<i32> = (0..size).collect();

    nums.shuffle(&mut rng);
    sorting::timsort::timsort(&mut nums);

    // println!("{nums:?}");
    assert_eq!(nums, sorted);
    println!("sorted!")
}