use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::max;
        use std::cmp::min;
        let mut area = 0;
        let mut start = 0;
        let mut end = height.len()-1;
        unsafe {
            while start < end {
                let width = end - start;
                let startValue = *height.get_unchecked(start);
                let endValue = *height.get_unchecked(end);
                if startValue <= endValue {
                    area = max(startValue * width as i32, area);
                    start += 1;
                } else {
                    area = max(endValue * width as i32, area);
                    end -= 1;
                }
            }
        }
        area
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod test{
    use crate::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}