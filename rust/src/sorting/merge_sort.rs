use std::cmp::min;


pub fn top_down_merge_sort<T: Ord + Copy>(nums: &mut [T]) {
	if nums.len() <= 1 {
		return;
	}

	let mid = nums.len() / 2;
	top_down_merge_sort(&mut nums[..mid]);
	top_down_merge_sort(&mut nums[mid..]);
	merge(nums, mid);
}

pub fn bottom_up_merge_sort<T: Ord + Copy>(nums: &mut [T]) {
	if nums.len() <= 1 {
		return;
	}

	let len = nums.len();
	let high_idx = len - 1;
	let mut subarray_len = 1;

	while subarray_len < len {
		let mut start = 0;
		while subarray_len < len - start {
			let end = min(start + 2*subarray_len, len);

			merge(&mut nums[start..end], subarray_len);

			start = end;
		}
		subarray_len *= 2;
	}
}

fn merge<T: Ord + Copy>(nums: &mut [T], mid: usize) {
	let left_half = nums[..mid].to_vec();
	let right_half = nums[mid..].to_vec();

	let mut l = 0;
	let mut r = 0;

	for num in nums {
		if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
			*num = left_half[l];
			l += 1;
		} else {
			*num = right_half[r];
			r += 1;
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    // top down (recursive) test cases
    #[test]
    fn test_top_down_merge_sort() {
        let mut nums = vec![7, 6, 3, 9, 2, 4, 1, 8, 10, 5];
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9 ,10]);
    }

    #[test]
    fn test_top_down_merge_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_top_down_merge_sort_one_element() {
        let mut nums = vec![1];
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    // bottom up (iterative) test cases
    #[test]
    fn test_bottom_up_merge_sort() {
        let mut nums = vec![7, 6, 3, 9, 2, 4, 1, 8, 10, 5];
        bottom_up_merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9 ,10]);
    }

    #[test]
    fn test_bottom_up_merge_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        bottom_up_merge_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_bottom_up_merge_sort_one_element() {
        let mut nums = vec![1];
        bottom_up_merge_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}