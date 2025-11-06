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
  - 様々な種類にコードの変形を行っていて参考になる。破壊的なアプローチを考えていなかった。
  Rust言語で考えるとメソッドのシグネチャ的にはRefCellで内部可変性を持つので破壊可能になっているという認識。呼び出し側から見ても引数が破壊的変更をされうることは分かる。
  個人的には参照が渡されるので破壊的なアプローチは取らないかなと思った。
  https://github.com/skypenguins/coding-practice/pull/22/files#diff-582300f2f8923de23d876e3fce724c4504135135c60d74ed1828d47534b52547R198

  - C++だとノードを共有することに忌避感を感じるとのことだったが、ポインタの管理(解放)を自分でする必要があるのが理由かなと考えた。
  Rustだと所有権システムによるスマートポインタで管理するので、このあたりの忌避感が自分に無い理由だと考えた。（コンパイラとのバトルが発生するが）
  https://github.com/5103246/LeetCode_Arai60/pull/22/files#diff-987fc900d5fa9b248277ec49bc97ea7a459549bf61ec2f2bb11c2fe9e532ab04R12

  改善する時に考えたこと
  - 特になし
*/

use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(node1) = root1 else {
            return root2;
        };
        let Some(node2) = root2 else {
            return Some(node1);
        };

        let (node1, node2) = (node1.borrow(), node2.borrow());
        let merged_root = Rc::new(RefCell::new(TreeNode::new(node1.val + node2.val)));

        merged_root.borrow_mut().left = Self::merge_trees(
            node1.left.as_ref().map(Rc::clone),
            node2.left.as_ref().map(Rc::clone),
        );
        merged_root.borrow_mut().right = Self::merge_trees(
            node1.right.as_ref().map(Rc::clone),
            node2.right.as_ref().map(Rc::clone),
        );

        Some(merged_root)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

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

    #[test]
    fn step2_test() {
        let root1 = vec_to_binary_tree(&vec![Some(1)]);
        let root2 = vec_to_binary_tree(&vec![Some(1), Some(2)]);
        let expect = vec_to_binary_tree(&vec![Some(2), Some(2)]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![Some(1)]);
        let root2 = vec_to_binary_tree(&vec![None]);
        let expect = vec_to_binary_tree(&vec![Some(1)]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![Some(1), Some(2), None, Some(3), None]);
        let root2 = vec_to_binary_tree(&vec![Some(1), None, Some(2), None, Some(4)]);
        let expect = vec_to_binary_tree(&vec![
            Some(2),
            Some(2),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
        ]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![Some(1), Some(2), None, Some(3), Some(4)]);
        let root2 = vec_to_binary_tree(&vec![None]);
        let expect = vec_to_binary_tree(&vec![Some(1), Some(2), None, Some(3), Some(4)]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![None]);
        let root2 = vec_to_binary_tree(&vec![None]);
        let expect = vec_to_binary_tree(&vec![None]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![]);
        let root2 = vec_to_binary_tree(&vec![]);
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);
    }
}
