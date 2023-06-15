use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn majority_element_tung(nums: Vec<i32>) -> i32 {
        let mut frequency_counter = HashMap::with_capacity(nums.len() / 2);
        for i in nums {
            let entry = frequency_counter.entry(i);
            let existing_frequency = entry.or_insert(0);
            *existing_frequency += 1;
        }
        frequency_counter.into_iter().max_by_key(|(value, freq)| {
            *freq
        }).unwrap().0
    }


    pub fn majority_element_ly(nums: Vec<i32>) -> i32 {
        let mut frequency_counter = HashMap::new();
        let mut max_frequency: i32 = 0;
        let mut majority_number = 0;
        for i in nums {
            let frequency = frequency_counter.entry(i).or_insert(0);
            *frequency += 1;
            if max_frequency < *frequency {
                max_frequency = *frequency;
                majority_number = i;
            }
        }
        majority_number
    }

    // pub fn majority_element_maro(nums: Vec<i32>) -> i32 {
    //     let mut freq = HashMap::new();
    //     //let mut max = 0;
    //     for num in nums {
    //         if !freq.contains_key(&num) {
    //             freq.insert(num, 0);
    //         } else {
    //                freq.get_mut(num).unwrap() += 1;
    //             freq.insert(num, freq.get(&num).unwrap()+1);
    //             if freq[&num] >= nums.len() / 2 {
    //                 return num
    //             }
    //         }
    //     }
    //     unimplemented!();
    // }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut data = nums;
        data.sort_unstable();
        data[data.len() / 2]
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
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let ret = Solution::majority_element(nums);
        println!("{:?}", ret);
    }
}
