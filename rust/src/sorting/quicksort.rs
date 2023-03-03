use std::cmp::PartialOrd;


pub fn quicksort<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(nums, low, high);
        quicksort(nums, low, pivot-1);
        quicksort(nums, pivot+1, high);
    }
}

fn partition<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut left = low;
    let mut right = high-1;

    loop {
        while nums[left] < nums[pivot] {
            left += 1;
        }
        while nums[right] > nums[pivot] {
            right -= 1;
        }

        if left >= right {
            break;
        } else {
            nums.swap(left, right);
        }
    }
    nums.swap(left, high);
    left
}
