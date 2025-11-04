// Step1_DFS
// 目的: 再帰による深さ優先探索の実装も練習しておく

/*
  所感
  - あまり自信が無いまま書いて通ったので、再帰処理はまだまだ苦手だなと思った。
  - 個人的にはキューを利用したBFSの実装が自然に書けると思った。
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::explore_max_depth(&root, 0)
    }

    fn explore_max_depth(node: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        let Some(node) = node else {
            return depth;
        };

        let left_depth = Self::explore_max_depth(&node.borrow().left.clone(), depth + 1);
        let right_depth = Self::explore_max_depth(&node.borrow().right.clone(), depth + 1);

        left_depth.max(right_depth)
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
    fn step1_bfs_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step1_bfs_test() {
        let node_values = vec![Some(3)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 1);

        let node_values = vec![Some(3), None, Some(5)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 2);

        let node_values = vec![];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 0);

        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 3);
    }
}
