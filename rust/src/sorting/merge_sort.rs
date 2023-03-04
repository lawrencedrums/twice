pub fn top_down_merge_sort<T: Ord + Copy>(nums: &mut [T]) {
	if nums.len() <= 1 {
		return;
	}

	let mid = nums.len() / 2;
	top_down_merge_sort(&mut nums[..mid]);
	top_down_merge_sort(&mut nums[mid..]);
	merge(nums, mid);
}

pub fn bottom_up_merge_sort<T: Ord + Copy>(nums: &mut [T]) {}

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

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 2, 4, 1, 5];
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_sort_one_element() {
        let mut nums = vec![1];
        top_down_merge_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}