// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nodes.len()
  時間計算量: O(n) 入力として与えられる木は二分木であることしかわからないので。完全二分木であれば深さlog nになるのでO(log n)になるという理解。
  空間計算量: O(n) 入力として与えられる木は二分木であることしかわからないので。直列な二分木であれば同時に処理するノードが1になるのでO(1)になるという理解。
*/

/*
  1回目: 4分48秒
  2回目: 3分46秒
  3回目: 3分53秒
*/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        let mut frontiers = VecDeque::new();
        let (lower_limit, upper_limit) = (i32::MIN as i64 - 1, i32::MAX as i64 + 1);

        frontiers.push_back((root, lower_limit, upper_limit));
        while let Some((node, lower_limit, upper_limit)) = frontiers.pop_front() {
            let node = node.borrow();
            let node_val = node.val as i64;
            let valid = lower_limit < node_val && node_val < upper_limit;
            if !valid {
                return false;
            }

            let (left_node, right_node) = (
                node.left.as_ref().map(Rc::clone),
                node.right.as_ref().map(Rc::clone),
            );
            if let Some(left_node) = left_node {
                frontiers.push_back((left_node, lower_limit, node_val));
            }
            if let Some(right_node) = right_node {
                frontiers.push_back((right_node, node_val, upper_limit));
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step3_test() {
        let root = vec_to_binary_tree(&vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::is_valid_bst(root), true);

        let root = vec_to_binary_tree(&vec![None]);
        assert_eq!(Solution::is_valid_bst(root), true);

        let root = vec_to_binary_tree(&vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert_eq!(Solution::is_valid_bst(root), false);

        let root = vec_to_binary_tree(&vec![Some(2), Some(3)]);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    fn vec_to_binary_tree(node_values: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root_value) = node_values.get(0).and_then(|v| *v) else {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(root_value)));
        let mut nodes = VecDeque::new();
        nodes.push_back(Rc::clone(&root));

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            let mut node = node.borrow_mut();
            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let left_node = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.left = Some(Rc::clone(&left_node));
                nodes.push_back(left_node);
            }
            i += 1;

            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let right_node = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.right = Some(Rc::clone(&right_node));
                nodes.push_back(right_node);
            }
            i += 1;
        }

        Some(root)
    }

    fn binary_tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut node_values = vec![];
        let mut nodes = VecDeque::new();
        nodes.push_back(root.as_ref().map(Rc::clone));

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            node_values.push(Some(node.borrow().val));
            nodes.push_back(node.borrow().left.as_ref().map(Rc::clone));
            nodes.push_back(node.borrow().right.as_ref().map(Rc::clone));
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn step3_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
