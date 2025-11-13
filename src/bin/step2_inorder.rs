// Step2
// 目的: 別の解法を写経する
// https://github.com/ryosuketc/leetcode_arai60/pull/29/files#diff-862aaa72d065e55c50b5f200702e6b643025add3aff25f967bcda516b10bd786

/*
  実装の理解
  - preorderを先頭から順番にrootノードとして扱う。
  - inorder[0]は一番左側のノードで、inorder.last()は一番右のノードになっている。
  - preorderはindexが増えるにつれて階層が深くなる。preorder[0]がrootでpreorder.last()が底。
  - あるノードをrootとして扱ったとき、rootノードの左右の子をinorderから探している。
  - ただし、preorder_indexはインクリメントしていくので、preorderにおいて通り過ぎた値は子になりえない。
  inorder,preorderのこのあたりの性質を理解して、さらに操作をコードに落とし込むのはかなり距離があるように感じる。

  所感
  - 書きやすい形に変形してみたが、再帰関数の基本ケース(returnするところ)でミスしてしまい、ChatGPT(GPT-5)を活用して修正した。
  何をしているのか理解できていないことが原因だと思う。
  時間切れなのでとりあえずここまで。
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

pub struct TreeBuilder {
    preorder: Vec<i32>,
    inorder: Vec<i32>,
    inorder_value_to_index: HashMap<i32, usize>,
}

impl TreeBuilder {
    pub fn new(preorder_node_values: Vec<i32>, inorder_node_values: Vec<i32>) -> Self {
        let inorder_value_to_index = inorder_node_values
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect();
        Self {
            preorder: preorder_node_values,
            inorder: inorder_node_values,
            inorder_value_to_index: inorder_value_to_index,
        }
    }

    pub fn build_tree(&self) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder_index = 0;
        let inorder_right_index = self.inorder.len() as isize - 1;
        self.build_tree_helper(0, inorder_right_index, &mut preorder_index)
    }

    fn build_tree_helper(
        &self,
        inorder_left_index: isize,
        inorder_right_index: isize,
        preorder_index: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder_left_index > inorder_right_index {
            return None;
        };
        // input_example: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
        let root_value = self.preorder.get(*preorder_index)?;
        let root = Rc::new(RefCell::new(TreeNode::new(*root_value)));
        let inorder_root_index = *self.inorder_value_to_index.get(root_value)? as isize;

        *preorder_index += 1;
        root.borrow_mut().left =
            self.build_tree_helper(inorder_left_index, inorder_root_index - 1, preorder_index);
        root.borrow_mut().right =
            self.build_tree_helper(inorder_root_index + 1, inorder_right_index, preorder_index);

        Some(root)
    }
}

pub struct Solution {}
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let tree_builder = TreeBuilder::new(preorder, inorder);
        tree_builder.build_tree()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step2_inorder_test() {
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
    fn step2_inorder_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
