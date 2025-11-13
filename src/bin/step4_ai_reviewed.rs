// Step4
// 目的: GPT-5によるレビューで指定された内容を修正

/*
  n = preorder.len() == inorder.len()
  時間計算量: O(n)
  空間計算量: O(n)
*/

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_value_to_index = inorder
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect::<HashMap<i32, usize>>();
        Self::build_tree_helper(&preorder, &inorder, &inorder_value_to_index)
    }

    fn build_tree_helper(
        preorder: &[i32],
        inorder: &[i32],
        inorder_value_to_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root_val) = preorder.get(0) else {
            return None;
        };
        let mut root = TreeNode::new(*root_val);

        let inorder_root_index = *inorder_value_to_index.get(root_val)?;
        // inorderは分割されてどんどん減っていくので、sliceの先頭から今見ているノードの値までの左部分木のノードを数える必要がある。このロジックは自分で書けずGPT-5に聞いた。
        let inorder_start_index = *inorder_value_to_index.get(&inorder[0])?;
        let inorder_left_length = inorder_root_index - inorder_start_index;

        let inorder_left = &inorder.get(0..inorder_left_length)?;
        let inorder_right = &inorder.get(inorder_left_length + 1..)?;

        let preorder_left = &preorder.get(1..inorder_left_length + 1)?;
        let preorder_right = &preorder.get(inorder_left_length + 1..)?;

        root.left = Self::build_tree_helper(preorder_left, inorder_left, &inorder_value_to_index);
        root.right =
            Self::build_tree_helper(preorder_right, inorder_right, &inorder_value_to_index);

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step4_test() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let expect = vec_to_binary_tree(&vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::build_tree(preorder, inorder), expect);

        let preorder = vec![-1];
        let inorder = vec![-1];
        let expect = vec_to_binary_tree(&vec![Some(-1)]);
        assert_eq!(Solution::build_tree(preorder, inorder), expect);

        let preorder = vec![];
        let inorder = vec![];
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::build_tree(preorder, inorder), expect);

        let preorder = vec![1];
        let inorder = vec![];
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::build_tree(preorder, inorder), expect);

        let preorder = vec![];
        let inorder = vec![1];
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::build_tree(preorder, inorder), expect);
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
    fn step4_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
