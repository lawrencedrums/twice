pub fn selection_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    for i in 0..nums.len() {
        let mut min = i;
        let mut j = i + 1;

        while j < nums.len() {
            if nums[j] < nums[min] {
                min = j;
            }
            j += 1;
        }

        if min != i {
            nums.swap(min, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 2, 4, 1, 5];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        selection_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_sort_one_element() {
        let mut nums = vec![1];
        selection_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}