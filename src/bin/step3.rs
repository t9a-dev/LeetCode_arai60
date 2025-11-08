// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = root.len() DFSなので根(root)から葉(leaf)までの高さとする
  時間計算量: O(n)
  空間計算量: O(n) 問題文にはbinary treeとしか記載されておらず直列になりえる。完全二分木だとO(log n)になるという認識。
*/

/*
  1回目: 2分4秒
  2回目: 1分50秒
  3回目: 1分52秒
*/

use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(node) = root else {
            return false;
        };
        let node = node.borrow();
        let is_leaf = node.left.is_none() && node.right.is_none();
        let target_sum = target_sum - node.val;

        if is_leaf {
            return target_sum == 0;
        }

        let (left_node, right_node) = (
            node.left.as_ref().map(Rc::clone),
            node.right.as_ref().map(Rc::clone),
        );

        Self::has_path_sum(left_node, target_sum) || Self::has_path_sum(right_node, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step3_has_target_sum_test() {
        let node_values = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 22), true);

        let node_values = vec![Some(1)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 1), true);

        let node_values = vec![Some(1), Some(0)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 1), true);

        let node_values = vec![Some(-1), Some(1)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), true);

        let node_values = vec![Some(-1), None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, -1), true);
    }

    #[test]
    fn step3_has_not_target_sum_test() {
        let node_values = vec![Some(1), Some(2), Some(3)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 5), false);

        let node_values = vec![None, None, None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);

        let node_values = vec![];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);

        let node_values = vec![None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);
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
