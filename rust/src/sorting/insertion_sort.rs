pub fn insertion_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    for i in 1..nums.len() {
        let mut j = i;
        let key = nums[i];

        while j > 0 && nums[j-1] > key {
            nums[j] = nums[j-1];
            j -= 1;
        }
        nums[j] = key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 2, 4, 1, 5];
        insertion_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        insertion_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_sort_one_element() {
        let mut nums = vec![1];
        insertion_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}