use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // allocation version
        let mut nums = nums;
        let n = n as usize;
        let mut nums : Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        // (0,2) (1,5) (2,1) (3,3)  ....
        nums.sort_by_key(|&(i, v)| {
            transform_index(i, n)
        });
        nums.into_iter().map(|(_, v)| v).collect()
    }
}


pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn transform_index(index: usize, n: usize) -> usize {
    if index < n {
        2 * index
    } else {
        let index = index - n;
        2 * index + 1
    }
}

fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
    let n = arr.len() / 2;
    let pivot = high as usize;
    let transformed_pivot = transform_index(pivot, n);
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        let mut transformed_store_index = transform_index(store_index as usize, n);
        while transformed_store_index < transformed_pivot {
            store_index += 1;
            transformed_store_index = transform_index(store_index as usize, n);
        }
        last_index -= 1;
        let mut transformed_last_index = transform_index(last_index as usize, n);
        while last_index >= 0 && transformed_last_index > transformed_pivot {
            last_index -= 1;
            transformed_last_index = transform_index(last_index as usize, n);
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot);
    store_index
}


#[cfg(test)]
mod test {
    use crate::shuffle_the_array::{quick_sort, transform_index};
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }

    #[test]
    fn test_sort() {
        let mut arr = [0,1,2,3,4,5];
        quick_sort(&mut arr);
        assert_eq!(arr, [0,3,1,4,2,5]);
    }

    #[test]
    fn test_index() {
        assert_eq!(transform_index(0, 3), 0);
        assert_eq!(transform_index(1, 3), 2);
        assert_eq!(transform_index(2, 3), 4);
        assert_eq!(transform_index(3, 3), 1);
        assert_eq!(transform_index(4, 3), 3);
        assert_eq!(transform_index(5, 3), 5);
    }
}
//leetcode submit region end(Prohibit modification and deletion)