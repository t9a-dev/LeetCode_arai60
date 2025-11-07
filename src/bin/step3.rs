// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums.len()
  時間計算量: O(n) 配列を分ける箇所で参照を使っており、値のコピーが発生しないのでO(1)となる。再帰でn回繰り返すのでO(n)
  空間計算量: O(log n) 値のコピーなどを行っておらず、平衡二分探索木なのでスタックの深さがlog nになる。
*/

/*
  1回目: 3分5秒
  2回目: 2分29秒
  3回目: 2分47秒
*/

use std::{cell::RefCell, rc::Rc};

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
        Self::build_binary_search_tree(&nums)
    }

    fn build_binary_search_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let nums_middle_index = nums.len() / 2;
        let node_value = nums.get(nums_middle_index)?;
        let (left_nums, right_nums) = (&nums[0..nums_middle_index], &nums[nums_middle_index + 1..]);
        let left_node = Self::build_binary_search_tree(left_nums);
        let right_node = Self::build_binary_search_tree(right_nums);

        let node = Rc::new(RefCell::new(TreeNode::new(*node_value)));
        {
            let mut node = node.borrow_mut();
            node.left = left_node;
            node.right = right_node;
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step3_test() {
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
    fn step3_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
