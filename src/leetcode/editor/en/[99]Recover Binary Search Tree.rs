// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use crate::Solution;
//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

fn traverse_tree(root: &TreeLink) -> Vec<TreeNode> {
    match root {
        None => Vec::new(),
        Some(root_inner) => {
            let left = root_inner.borrow();
            let right = root_inner.borrow();
            let mid = root_inner.borrow();
            if mid.val >= left.val && mid.val < right.val {
                traverse_tree(&mid.left).append(&mut traverse_tree(&mid.right));
            }
            unimplemented!();
        }
    }
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {}
}
//leetcode submit region end(Prohibit modification and deletion)
