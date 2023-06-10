use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        unsafe { Solution::lcm(text1.as_str(), text2.as_str()) }
    }

    unsafe fn lcm(text1: &str, text2: &str) -> i32 {
        use std::cmp::max;
        let chars1: Vec<char> = text1.chars().collect();
        let chars2: Vec<char> = text2.chars().collect();
        let mut table = vec![vec![0; chars2.len() + 1]; chars1.len() + 1];
        for (i, &c1) in chars1.iter().enumerate() {
            for (j, &c2) in chars2.iter().enumerate() {
                let i = i + 1;
                let j = j + 1;
                *table.get_unchecked_mut(i).get_unchecked_mut(j) = if c1 == c2 {
                    table[i - 1][j - 1] + 1
                } else {
                    max(table[i - 1][j], table[i][j - 1])
                }
            }
        }
        table[chars1.len()][chars2.len()]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


// generate unit test
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_common_subsequence(
            String::from("oxcpqrsvwf"), String::from("shmtulqrypy")), 2);
    }
}