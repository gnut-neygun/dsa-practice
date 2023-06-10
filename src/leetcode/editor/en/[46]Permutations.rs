use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            return vec![nums];
        }
        let k = nums.len();
        Solution::permute_rec(nums, k)
    }

    fn permute_rec(nums: Vec<i32>, k: usize) -> Vec<Vec<i32>>{
        if k == 1 {
            return vec![nums];
        }
        let mut ret = Solution::permute_rec(nums.clone(), k - 1);
        for i in 0..k-1{
            let mut clone = nums.clone();
            clone.swap(i, k - 1);
            ret.append(&mut Solution::permute_rec(clone, k - 1));
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        let solution = Solution::permute(vec![1, 2, 3]);
        assert_eq!(solution.len(), 6);
        println!("{:?}", solution)
    }
}