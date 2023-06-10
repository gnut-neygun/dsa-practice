use std::ptr::hash;

use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result: Vec<bool> = Vec::new();
        result.reserve(candies.len());
        let mut max = 0;
        for &i in candies.iter() {
            if i > max {
                max = i;
            }
        }
        for &i in candies.iter() {
            result.push(i + extra_candies >= max);
        }

        result
    }

    #[inline]
    pub fn kids_with_candices_2(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies.into_iter().map(|kid_candy| {
            kid_candy + extra_candies >= max
        }).collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::kids_with_candices_2(vec![2, 3, 5, 1, 3], 3), vec![true, true, true,
                                                                                false, true]);
        assert_eq!(Solution::kids_with_candices_2(vec![4, 2, 1, 1, 2], 1), vec![true, false, false,
                                                                                false, false]);
        assert_eq!(Solution::kids_with_candices_2(vec![12, 1, 12], 10), vec![true, false, true]);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3), vec![true, true, true,
                                                                             false, true]);
        assert_eq!(Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1), vec![true, false, false,
                                                                             false, false]);
        assert_eq!(Solution::kids_with_candies(vec![12, 1, 12], 10), vec![true, false, true]);
    }
}