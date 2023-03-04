pub fn bubble_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }

    loop {        
        let mut swapped = false;

        for i in 1..nums.len() {
            if nums[i-1] > nums[i] {
                nums.swap(i-1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec![3, 5, 1, 9, 6];
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![1, 3, 5, 6, 9]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_sort_one_element() {
        let mut nums = vec![1];
        bubble_sort(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}