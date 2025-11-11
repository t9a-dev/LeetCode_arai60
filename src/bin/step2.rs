// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - lower_limit,upper_limit変数名について別のものを提案されるかも

  他の人のコードを読んで考えたこと
  - rootがNoneとき二分木ではないだろうと思ってstep1ではfalseを返していたが、Noneでも有効な二分木として扱うのでtrueを返すそう。
  直感に反するので知識として覚えておく必要があると思った。
  https://github.com/potrue/leetcode/pull/28/files#r2153779987

  - inorderによる解法。yieldがよく分からず時間がかかりそうなのでスキップするが、解法にバリエーションがあるということで。
  https://github.com/docto-rin/leetcode/pull/33/files#diff-9fdcc2342d5e1d4a23edd1bd1fd71af2b84024472dec032836dcd610e914dd0fR46

  - 解法のバリエーションが多くて良いと思った。
  https://github.com/hayashi-ay/leetcode/pull/38/files

  改善する時に考えたこと
  - 入力のrootがNoneであるときはTrueを返す。https://github.com/potrue/leetcode/pull/28/files#r2153779987
  - lower_limit,upper_limitは他のコードで見かけなかったが、個人的に分かりやすいと思うのでこのままにする。

  所感
  - 最初問題を見たときは再帰によるDFSで行ける！と思ったが、全然だめだったし、BFSの方が素直に書けると思った。
    - 自分が採用している解法だと下限値・上限値を引き継ぐ必要があるため、思ったより再帰ですっきりと書けないんだろうなと思った。
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
                frontiers.push_back((Rc::clone(&left_node), lower_limit, node_val));
            }
            if let Some(right_node) = right_node {
                frontiers.push_back((Rc::clone(&right_node), node_val, upper_limit));
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
    fn step2_test() {
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
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
