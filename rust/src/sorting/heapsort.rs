pub fn heapsort(nums: &mut Vec<i32>) {
    let heap_size = nums.len();

    // build heap
    for i in (0..(heap_size/2)).rev() {
        heapify(nums, heap_size, i);
    }

    for i in (0..heap_size).rev() {
        nums.swap(0, i);

        // heapify root element to get highest element at root again
        heapify(nums, i, 0);
    }
}

fn heapify(nums: &mut Vec<i32>, heap_size: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // find largest child
    if left < heap_size && nums[left] > nums[largest] {
        largest = left;
    }
    if right < heap_size && nums[right] > nums[largest] {
        largest = right;
    }
    // swap and continue heapify if root is not largest
    if largest != i {
        nums.swap(i, largest);
        heapify(nums, heap_size, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut nums = vec!(2, 4, 5, 1, 9, 3, 8, 6, 7, 10);
        heapsort(&mut nums);
        assert_eq!(nums, vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    }

    #[test]
    fn test_sort_empty_array() {
        let mut nums = Vec::<i32>::new();
        heapsort(&mut nums);
        assert_eq!(nums, vec!());
    }

    #[test]
    fn test_sort_single_element() {
        let mut nums = vec!(1);
        heapsort(&mut nums);
        assert_eq!(nums, vec!(1));
    }
}
