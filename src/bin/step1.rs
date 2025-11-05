// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 二分木が与えられるので根からリーフ（子を持たないノード）までの最小深さを返す。

  何がわからなかったか
  - 特になし

  何を考えて解いていたか
  - リーフはleft,rightともにNoneとなることが条件なので、BFSでこの条件に一致するノード（リーフ）を見つけたら即リターンで結果を返す。

  正解してから気づいたこと
  - 直前に解いた「104.Maximum Depth of Binary Tree」とほぼ同じコードなこともあり、あまり何も思わなかった。
  - 練習として再帰処理も書いておく(step1_DFS.rs)
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
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }

            frontiers.push_back((node.left.as_ref().map(Rc::clone), depth + 1));
            frontiers.push_back((node.right.as_ref().map(Rc::clone), depth + 1));
        }

        0
    }
}

#[cfg(test)]
mod tests {
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
    fn step1_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step1_test() {
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
