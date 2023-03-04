use std::cmp::PartialOrd;


pub fn quicksort<T: Ord>(nums: &mut [T]) {
    let len = nums.len();
    if len > 1 {
        quicksort_helper(nums, 0, (len - 1) as isize);
    }
}

fn quicksort_helper<T: Ord>(nums: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot = partition(nums, low, high);
        quicksort_helper(nums, low, pivot-1);
        quicksort_helper(nums, pivot+1, high);
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 2, 4, 1, 5];
        quicksort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        quicksort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_sort_one_element() {
        let mut nums = vec![1];
        quicksort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}