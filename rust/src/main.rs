use rand::seq::SliceRandom;
use rand::thread_rng;

mod sorts;


fn main() {
    let mut rng = thread_rng();
    let mut nums: Vec<u32> = (0..20).collect();
    let sorted: Vec<u32> = (0..20).collect();

    nums.shuffle(&mut rng);
    // sorts::bubble_sort::bubble_sort(&mut nums);
    // sorts::selection_sort::selection_sort(&mut nums);
    sorts::insertion_sort::insertion_sort(&mut nums);
   
    println!("{:?}", nums);
    assert_eq!(nums, sorted);
}
