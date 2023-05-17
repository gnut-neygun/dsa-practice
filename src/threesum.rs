use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut hash_set: HashSet<Vec<i32>> = HashSet::new();
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut index_map = HashMap::<i32, Vec<usize>>::new();
        for (index, i) in nums.iter().enumerate() {
            let value = index_map.entry(*i).or_insert(Vec::new());
            value.push(index);
        }
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                let to_search = 0 - nums[i] - nums[j];
                match index_map.get(&to_search) {
                    None => (),
                    Some(vec) => {
                        vec.iter().filter(|value| **value > j).map(|index| {
                            let mut ret = vec![nums[i], nums[j], nums[*index]];
                            ret.sort();
                            ret
                        }).for_each(|value| {
                            hash_set.insert(value);
                        });
                    }
                }
            }
        }
        Vec::from_iter(hash_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ret = Solution::three_sum(nums);
        println!("{:?}", ret);
    }
}