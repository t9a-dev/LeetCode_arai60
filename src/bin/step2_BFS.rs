// Step2_BFS
// 目的: step1_BFSの改善を考える

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

/*
  改善する時に考えたこと
  - 配列中央を表す単語はcenter -> middleにする。step2.rsの改善と同じ。
  - numsを分割して取り回すときにclone()しているが参照で取り回せないか？
  clone()する対象がプリミティブな値(i32)なのでコストを気にすることは無いが、numsは変更しないので不要なcloneだと思った。

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
        let Some(root_node_value) = Self::get_middle_value(&nums) else {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(*root_node_value)));
        let mut frontiers = VecDeque::new();

        frontiers.push_back(((Self::split_left_and_right(&nums)), Rc::clone(&root)));
        while let Some(((left_nums, right_nums), node)) = frontiers.pop_front() {
            if let Some(node_value) = Self::get_middle_value(left_nums) {
                let left_node = Rc::new(RefCell::new(TreeNode::new(*node_value)));
                node.borrow_mut().left = Some(Rc::clone(&left_node));
                frontiers.push_back((Self::split_left_and_right(left_nums), left_node));
            }

            if let Some(node_value) = Self::get_middle_value(right_nums) {
                let right_node = Rc::new(RefCell::new(TreeNode::new(*node_value)));
                node.borrow_mut().right = Some(Rc::clone(&right_node));
                frontiers.push_back((Self::split_left_and_right(right_nums), right_node));
            }
        }

        Some(root)
    }

    fn get_middle_value(nums: &[i32]) -> Option<&i32> {
        nums.get(nums.len() / 2)
    }

    fn split_left_and_right(nums: &[i32]) -> (&[i32], &[i32]) {
        let middle_index = nums.len() / 2;
        (&nums[0..middle_index], &nums[middle_index + 1..])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step2_bfs_test() {
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
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
