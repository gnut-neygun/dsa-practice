use crate::Solution;
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
//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

fn is_same_tree_rec(p: &TreeLink, q: &TreeLink) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(rootx), Some(rooty)) => {
            let borrowed_x = rootx.borrow();
            let borrowed_y = rooty.borrow();
            borrowed_x.val == borrowed_y.val && is_same_tree_rec(&borrowed_x.left, &borrowed_y.left) &&
                is_same_tree_rec(&borrowed_x.right, &borrowed_y.right)
        }
    }
}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) ->
    bool {
        is_same_tree_rec(&p, &q)
    }
}
//leetcode submit region end(Prohibit modification and deletion)