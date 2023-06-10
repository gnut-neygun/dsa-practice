use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
struct PermuteIter{
    array: Vec<i32>,
    n: usize,
    i: usize
}

impl Iterator for PermuteIter{
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.n{
            return None;
        }
        let mut clone = self.array.clone();
        clone.swap(self.n, self.i);
        Some(clone)
    }
}

fn permute_rec(nums: Vec<i32>, n: usize) -> PermuteIter {
    let iterator = permute_rec(nums, nums.len() - 1);
    for v in iterator{

    }

    unimplemented!();
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let iterator = permute(nums)
        unimplemented!();
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::permute(vec![1, 2, 3]), vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }
}