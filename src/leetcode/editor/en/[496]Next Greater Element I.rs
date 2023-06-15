use crate::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater_element_map = HashMap::new();
        for (i, &value) in nums2.iter().enumerate() {
            let greater_element = nums2[i + 1..].into_iter().find(|x| **x > value);
            match greater_element {
                None => {
                    next_greater_element_map.insert(value, -1);
                }
                Some(&k) => {
                    next_greater_element_map.insert(value, k);
                }
            };
        }
        nums1.into_iter().map(|x| {
            *next_greater_element_map.get(&x).unwrap()
        }).collect()
    }

    pub fn next_greater_element_hieu(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut m = HashMap::new();
        for (i, &v) in nums2.iter().enumerate() {
            m.insert(v, i);
        }

        for &num in nums1.iter() {
            let mut check = false;
            let index = *m.get(&num).unwrap();
            for &v in nums2[index..].iter() {
                if v > num {
                    res.push(v);
                    check = true;
                    break;
                }
            }
            if !check {
                res.push(-1);
            }
        }
        res
    }


    pub fn next_greater_element_ly(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // // nums1 = [4,1,2]   nums2 = [1,4,2,0,5,3]
        // first run from 0 to n-1 of nums2
        // if current_number < next_number then hash[current_number] = next_number

        // we have: hash[1] = 4, hash[0] = 5, hash[3] = -1

        // second loop from n-1 to 0
        // if current_number > next_number, hash[next_number] = hash[current_number]
        // hash[5] = hash[-3] = -1
        // hash[2] = hash[0] = 5
        // hash[4] = hash[2] = 5
        let mut next_greater_hashmap = HashMap::new();
        let mut next_greater_number = vec![-1; nums1.len()];

        next_greater_hashmap.insert(nums2[nums2.len() - 1], -1);
        for i in 0..(nums2.len() - 1) {
            if nums2[i] < nums2[i + 1] {
                next_greater_hashmap.insert(nums2[i], nums2[i + 1]);
            }
        }
        for i in (0..(nums2.len() - 1)).rev() {
            if nums2[i] > nums2[i + 1] {
                if next_greater_hashmap.get(&nums2[i]).is_none() {
                    next_greater_hashmap.insert(nums2[i], *next_greater_hashmap.get(&nums2[i + 1]).unwrap());
                }

            }
        }
        for i in 0..nums1.len() {
            next_greater_number[i] = *next_greater_hashmap.get(&nums1[i]).unwrap_or(&-1);
        }
        next_greater_number
    }


    pub fn next_greater_element_dd(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut num2HashMap = HashMap::new();
        let default = -1;
        for i in 0..(nums2.len() - 2) {
            if nums2[i] < nums2[i + 1] {
                num2HashMap.insert(nums2[i], nums2[i + 1]);
            }
        }

        nums1.into_iter().map(|x| {
            *num2HashMap.get(&x).unwrap_or(&default)
        }).collect()

    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_next_greater_element() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let ret = Solution::next_greater_element(nums1, nums2);
        println!("{:?}", ret);
    }

    #[test]
    fn test_next_greater_element_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let ret = Solution::next_greater_element(nums1, nums2);
        println!("{:?}", ret);
    }
}
