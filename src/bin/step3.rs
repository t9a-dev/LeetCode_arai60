// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  V = ノード(頂点)の数
  時間計算量: O(V)
  空間計算量: O(V)
*/

/*
  1回目: 2分40秒
  2回目: 3分12秒
  3回目: 2分39秒
*/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
        let mut frontiers = VecDeque::new();

        frontiers.push_back((root, 1));
        while let Some((node, depth)) = frontiers.pop_front() {
            let Some(node) = node else {
                continue;
            };

            let node = node.borrow();
            let is_leaf = node.left.is_none() && node.right.is_none();
            if is_leaf {
                return depth;
            };

            frontiers.push_back((node.left.as_ref().map(Rc::clone), depth + 1));
            frontiers.push_back((node.right.as_ref().map(Rc::clone), depth + 1));
        }

        0
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
    fn step3_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step3_test() {
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
