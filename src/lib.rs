#![allow(unused_variables)]
#![allow(dead_code)]

use std::future::Future;
use std::pin::Pin;
use std::process::Output;
use std::task::{Context, Poll};

struct Solution;

//#[path = "leetcode/editor/en/[5]Longest Palindromic Substring.rs"]
//mod palindrome_substring;
#[path = "leetcode/editor/en/[15]3Sum.rs"]
mod three_sum;
#[path = "leetcode/editor/en/[16]3Sum Closest.rs"]
mod three_sum_closest;
#[path = "leetcode/editor/en/[1143]Longest Common Subsequence.rs"]
mod longest_common_subsequence;
#[path = "leetcode/editor/en/[11]Container With Most Water.rs"]
mod container_most_water;
#[path = "leetcode/editor/en/[46]Permutations.rs"]
mod permutations;
#[path = "leetcode/editor/en/[1470]Shuffle the Array.rs"]
mod shuffle_the_array;
#[path = "leetcode/editor/en/[1431]Kids With the Greatest Number of Candies.rs"]
mod kids_with_candies;