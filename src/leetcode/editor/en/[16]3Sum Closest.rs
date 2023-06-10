use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            unreachable!();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2] - target;
        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let difference = nums[i] + nums[j] + nums[k] - target;
                closest = if difference.abs() < closest.abs() {
                    difference
                } else {
                    closest
                };
                match difference.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        j += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        k -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        return difference + target;
                    }
                }
            }
        }
        closest + target
    }
}

trait Aboslute {
    fn abs(&self) -> Self;
}

impl Aboslute for i32 {
    fn abs(&self) -> Self {
        if *self < 0 {
            -*self
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test2() {
        let arr = vec![0, 3, 97, 102, 200];
        let target = 300;
        let ret = Solution::three_sum_closest(arr, target);
        assert_eq!(ret, 300)
    }
}
//leetcode submit region end(Prohibit modification and deletion)