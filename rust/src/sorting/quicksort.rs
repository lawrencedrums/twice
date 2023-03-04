use std::cmp::PartialOrd;


pub fn quicksort<T: PartialOrd>(nums: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot = partition(nums, low, high);
        quicksort(nums, low, pivot-1);
        quicksort(nums, pivot+1, high);
    }
}

// TODO: refactor this
fn partition<T: PartialOrd>(nums: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut left = low - 1;
    let mut right = low;

    while right < high {
        if nums[right as usize] < nums[pivot as usize] {
            left += 1;
            nums.swap(left as usize, right as usize);
        }
        right += 1;
    }

    nums.swap((left + 1) as usize, pivot);
    left + 1
}
