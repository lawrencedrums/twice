use std::cmp::PartialOrd;


pub fn quicksort<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(nums, low, high);
        quicksort(nums, low, pivot-1);
        quicksort(nums, pivot+1, high);
    }
}

// TODO: refactor this
fn partition<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut left = low - 1;
    let mut right = high;

    loop {
        left += 1;
        while nums[left] < nums[pivot] {
            left += 1;
        }

        right -= 1;
        while right > 0 && nums[right] > nums[pivot] {
            right -= 1;
        }

        if left >= right {
            break;
        } else {
            nums.swap(left, right);
        }
    }
    nums.swap(left, pivot);
    left
}
