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
  他の人のコードを読んで考えたこと
  - 分かりやすい言語化だと思った。
  https://github.com/colorbox/leetcode/pull/43/files#diff-11d4ee5278f48bebcb1b8cc444c5b5617b861f3dea5dc300fc2fff78418ac36dR7
  > inorderの、特定のノードを境に左部分木と右部分木に分かれる性質を活用する。

  - コメント集
  どれも理解できなかった。
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0#heading=h.1rv0z8fm6lc3

  - 別の解法を理解することで別の視点から問題を考えられるので理解が深まると思ったが、そもそも実装で何をしているのかが理解できなかった。
  C++だから読めないのではなくで、なぜそこでそうするのか、どういった気持ち（動機）でその処理をしているのかが理解できなかった。
  https://github.com/kazukiii/leetcode/pull/30#discussion_r1821628634
  ほぼ同じ感想だった。
  https://github.com/ryosuketc/leetcode_arai60/pull/29/files#diff-e680f8c547ec960a53ea69c35a7e7de8e18eaf790b3c8cc738f164555df54a58R26

  改善する時に考えたこと
  - inorder_middle_index -> inorder_root_index
  - .to_vec()によるコピーを無くす。
  - build_tree_helperの戻り値がOptionなので preorder.get(0)? で値が取得できなければreturn Noneとなるので、再帰関数の基本ケースとなっているが、
  読み手によっては無限ループになっているように見えると思うので、あえて preorder.get(0) else { return None; };と省略しない形にする。

  所感
  - 答えを見ずに一発で書けるものの、preorder,inorderがどのような並び順でノードが入っているということが完全に理解できていないせいか、テストケースの入力値を見ながら実装しないと何をしているかわからなくなる。
    - こうしたらこうなるという感じで実装している感覚。
  - 一つぐらいは別の解法をみておきたいので、以下の実装を写経する。(step2_inorder.rs)
  https://github.com/ryosuketc/leetcode_arai60/pull/29/files#diff-862aaa72d065e55c50b5f200702e6b643025add3aff25f967bcda516b10bd786
*/
use std::{cell::RefCell, rc::Rc};

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
        Self::build_tree_helper(&preorder, &inorder)
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root_val) = preorder.get(0) else {
            return None;
        };
        let mut root = TreeNode::new(*root_val);

        let inorder_root_index = inorder.iter().position(|v| v == root_val)?;
        let inorder_left = &inorder[0..inorder_root_index];
        let inorder_right = &inorder[inorder_root_index + 1..];

        let preorder_left = &preorder[1..inorder_left.len() + 1];
        let preorder_right = &preorder[inorder_left.len() + 1..];

        root.left = Self::build_tree_helper(&preorder_left, &inorder_left);
        root.right = Self::build_tree_helper(&preorder_right, &inorder_right);

        Some(Rc::new(RefCell::new(root)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step2_test() {
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
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
