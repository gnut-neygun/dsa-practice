use std::cmp::max;
use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let mut ret_set: HashSet<[i32; 3]> = HashSet::new();
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort();
        let max = nums[nums.len() - 1];
        let mut checked = HashSet::<i32>::new();
        for i in 0..nums.len() - 2 {
            let first = nums[i];
            if checked.contains(&first) || first > 0 || first < -2 * max {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            checked.insert(first);
            while  j < k {
                let second = nums[j];
                let third = nums[k];
                match first + second + third {
                    0 => {
                        ret_set.insert([first, second, third]);
                        j += 1;
                        k -= 1;
                    }
                    x if x > 0 => {
                        k -= 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
        }
        Vec::from_iter(ret_set.into_iter().map(Vec::from))
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ret = Solution::three_sum(nums);
        println!("{:?}", ret);
    }

    #[test]
    fn test_three_sum_2() {
        let nums = vec![3, 0, -2, -1, 1, 2];
        let ret = Solution::three_sum(nums);
        println!("{:?}", ret);
    }
}
//leetcode submit region end(Prohibit modification and deletion)