use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        unimplemented!();
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_majority_element() {
        let nums = vec![3, 2, 3];
        let ret = Solution::majority_element(nums);
        println!("{:?}", ret);
    }

    #[test]
    fn test_majority_element_2() {
        let nums = vec![1, 1, 1, 3, 3, 2, 2, 2];
        let ret = Solution::majority_element(nums);
        println!("{:?}", ret);
    }
}
