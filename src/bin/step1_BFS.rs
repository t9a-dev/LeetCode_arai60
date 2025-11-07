// Step1_BFS
// 目的: 練習のためにBFSで実装する

/*
  問題の理解
  - 昇順ソートされた整数を持つ配列numsが与えられる。numsをheight-balanced binary search tree に変換して返す。
  height-balanced binary search treeは初めて聞いた。
  平衡二分探索木というもので、どのノードでも左右の木の高さになるべく差が出ないようにしている二分木のこと。
  https://ja.wikipedia.org/wiki/%E5%B9%B3%E8%A1%A1%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2%E6%9C%A8

  所感
  - BFSとキューを利用した解法はすんなりと書けた。これまで解いてきた木構造を扱う問題とほとんど同じコードになっている。
  あるデータ構造を操作するコードにはある程度パターンがあって、この操作のパターンで何をしているのかを理解さえしていれば変形して様々な問題に対応できるのかなどと考えた。
  文章にしてみると当たり前のことしか書いていない気がするが、問題を一発で解けるかどうかに関わらず自分の反応が変化していることを感じた。
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root_value) = Self::get_center_value(&nums) else {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(root_value)));
        let mut frontiers = VecDeque::new();

        frontiers.push_back((Self::split_left_and_right(nums), Rc::clone(&root)));
        while let Some(((left_values, right_values), node)) = frontiers.pop_front() {
            if let Some(left_value) = Self::get_center_value(&left_values) {
                let left_node = Rc::new(RefCell::new(TreeNode::new(left_value)));
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                frontiers.push_back((Self::split_left_and_right(left_values), left_node));
            }

            if let Some(right_value) = Self::get_center_value(&right_values) {
                let right_node = Rc::new(RefCell::new(TreeNode::new(right_value)));
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                frontiers.push_back((Self::split_left_and_right(right_values), right_node));
            }
        }

        Some(root)
    }

    fn split_left_and_right(nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let center = nums.len() / 2;
        (nums[0..center].to_vec(), nums[center + 1..].to_vec())
    }

    fn get_center_value(nums: &Vec<i32>) -> Option<i32> {
        nums.get(nums.len() / 2).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step1_bfs_test() {
        let node_values = vec![-10, -3, 0, 5, 9];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)];
        assert_eq!(actual, expect);

        let node_values = vec![3, 1];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(1), Some(3)];
        assert_eq!(actual, expect);

        let node_values = vec![1];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(1)];
        assert_eq!(actual, expect);

        let node_values = vec![];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![];
        assert_eq!(actual, expect);

        let node_values = vec![-15, -10, -3, 0, 5, 9, 6];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![
            Some(0),
            Some(-10),
            Some(9),
            Some(-15),
            Some(-3),
            Some(5),
            Some(6),
        ];
        assert_eq!(actual, expect);
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
    fn step1_bfs_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
