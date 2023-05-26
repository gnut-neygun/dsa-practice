#![allow(unused_variables)]
#![allow(dead_code)]

use std::future::Future;
use std::pin::Pin;
use std::process::Output;
use std::task::{Context, Poll};

struct Solution;

mod safe_disjoint_union;
mod disjoint_union;
mod threesum;
//#[path = "leetcode/editor/en/[5]Longest Palindromic Substring.rs"]
//mod palindrome_substring;
#[path = "leetcode/editor/en/[15]3Sum.rs"]
mod three_sum;