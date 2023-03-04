use rand::seq::SliceRandom;
use rand::thread_rng;

mod sorting;


fn main() {
    let size = 200;

    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (0..size).collect();
    let sorted: Vec<i32> = (0..size).collect();

    nums.shuffle(&mut rng);
    // sorting::bubble_sort::bubble_sort(&mut nums);
    // sorting::selection_sort::selection_sort(&mut nums);
    // sorting::insertion_sort::insertion_sort(&mut nums);
    sorting::quicksort::quicksort(&mut nums, 0, (size-1) as isize);
    // sorting::merge_sort::top_down_merge_sort(&mut nums);

    // println!("{nums:?}");
    assert_eq!(nums, sorted);
    println!("sorted!")
}
