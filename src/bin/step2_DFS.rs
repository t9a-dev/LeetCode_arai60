// Step2_DFS
// 目的: step1_DFSの自然な書き方を練習

/*
  他の人のコードを読んで考えたこと
  - 再帰用のメソッドを別に作成しなくても良いことを以下のコードを見て気がついた。
  https://github.com/quinn-sasha/leetcode/pull/21/files#diff-c2ef246709da963a01dc6a50ecb5b5ce1169312bb960fb96731d87c76e1aa8a4R31-R42

  所感
  - 分からなかったのは、再帰処理中にノードが無いときにreturn する値とその値をreturnしたときに帰りがけでどのように処理するかだと思った。
  正直答えを見ても自分で書ける気がしない。
  特にleft_depth == 0, right_depth == 0でそれぞれ逆のノードの深さを足している部分が直感的に分からない。
  - このコードを読んだときにleaf（子を持たないノード）を探していることは一発でわからないと思った。
*/

use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };

        let node = node.borrow();
        let left_depth = Self::min_depth(node.left.as_ref().map(Rc::clone));
        let right_depth = Self::min_depth(node.right.as_ref().map(Rc::clone));

        if left_depth == 0 {
            return right_depth + 1;
        }
        if right_depth == 0 {
            return left_depth + 1;
        }

        left_depth.min(right_depth) + 1
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn to_tree_nodes(node_values: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if node_values.is_empty() || node_values[0].is_none() {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(node_values[0].unwrap())));
        let mut nodes = VecDeque::new();
        nodes.push_back(Rc::clone(&root));

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let left = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                nodes.push_back(left);
            }
            i += 1;

            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let right = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                nodes.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    fn to_node_values(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut node_values = Vec::new();
        let mut nodes = VecDeque::new();

        nodes.push_back(root.clone());
        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            node_values.push(Some(node.borrow().val));
            nodes.push_back(node.borrow().left.clone());
            nodes.push_back(node.borrow().right.clone());
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn step2_dfs_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step2_dfs_test() {
        let node_values = vec![Some(3)];
        let expect = 1;
        assert_eq!(Solution::min_depth(to_tree_nodes(&node_values)), expect);

        let node_values = vec![Some(3), None, Some(5)];
        let expect = 2;
        assert_eq!(Solution::min_depth(to_tree_nodes(&node_values)), expect);

        let node_values = vec![];
        let expect = 0;
        assert_eq!(Solution::min_depth(to_tree_nodes(&node_values)), expect);

        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let expect = 2;
        assert_eq!(Solution::min_depth(to_tree_nodes(&node_values)), expect);

        let node_values = vec![
            Some(3),
            None,
            Some(20),
            None,
            Some(10),
            None,
            Some(7),
            None,
            Some(42),
        ];
        let expect = 5;
        assert_eq!(Solution::min_depth(to_tree_nodes(&node_values)), expect);
    }
}
