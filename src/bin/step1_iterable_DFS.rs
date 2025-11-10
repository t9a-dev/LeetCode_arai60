// Step1_iterable_DFS
// 目的: iterable DFSの実装を行う

/*
  問題の理解
  - 二分木の根が与えられるので階層ごとにノードを配列にまとめて返す。左側優先探索から開始して階層ごとに左右の探索方向をジグザグに切り替える。
  以下の二分木の場合 [[1],[3,2],[4,5,6,7]] となる。
      1
     / \
    2   3
   / \ / \
  4  5 6  7

  [[1],[3,2],[4,5]]
      1
     / \
    2   3
   /     \
  4       5

  所感
  - step1.rs -> step1_DFS.rs -> 本ファイルと三回実装して気付いたが、変数zigzag -> reversingの方が分かりやすい気がするのでstep2で変更する。
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut zigzag_level_order_nodes: Vec<VecDeque<_>> = Vec::new();
        let mut frontiers = Vec::new();

        frontiers.push((root, 0));
        while let Some((node, level)) = frontiers.pop() {
            let node = node.borrow();
            let zigzag = level % 2 != 0;

            match zigzag_level_order_nodes.get_mut(level) {
                Some(nodes) if zigzag => nodes.push_front(node.val),
                Some(nodes) if !zigzag => nodes.push_back(node.val),
                None => zigzag_level_order_nodes.push(VecDeque::from_iter([node.val])),
                _ => unreachable!(),
            }

            let (node_left, node_right) = (
                node.left.as_ref().map(Rc::clone),
                node.right.as_ref().map(Rc::clone),
            );

            if let Some(node) = node_right {
                frontiers.push((node, level + 1));
            }
            // left-first
            if let Some(node) = node_left {
                frontiers.push((node, level + 1));
            }
        }

        zigzag_level_order_nodes
            .into_iter()
            .map(|v| v.into_iter().collect::<Vec<_>>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let root = vec_to_binary_tree(&vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expect = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![
            Some(3),
            Some(9),
            Some(20),
            Some(4),
            None,
            None,
            Some(7),
        ]);
        let expect = vec![vec![3], vec![20, 9], vec![4, 7]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![Some(3)]);
        let expect = vec![vec![3]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![None]);
        let expect: Vec<Vec<_>> = Vec::new();
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![]);
        let expect: Vec<Vec<_>> = Vec::new();
        assert_eq!(Solution::zigzag_level_order(root), expect);
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
    fn step1_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
