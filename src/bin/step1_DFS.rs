// Step1
// 目的: 再帰処理の実装練習

/*
  正解してから気づいたこと
  - 再帰処理にすることで得られる、シンプルな記述になっていない気がする。特に外側からmin_depthを渡している点。
  この感覚が正しいのかも微妙。再帰処理でDFSを実装した場合に、再帰処理の外側で状態を持たずに一番浅いリーフを探索する方法が思いつかない。
  一応ChatGPT(GPT-5)に聞いて、より良い実装が出てきたらstep2_DFS.rsに写経だけする。
*/

use std::{cell::RefCell, rc::Rc};

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
        if root.is_none() {
            return 0;
        };

        let mut min_depth: Option<i32> = None;
        Self::explore_min_depth(root, 1, &mut min_depth);

        min_depth.unwrap_or(0)
    }

    fn explore_min_depth(
        node: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        min_depth: &mut Option<i32>,
    ) {
        let Some(node) = node else {
            return;
        };

        let node = node.borrow();
        if node.left.is_some() || node.right.is_some() {
            Self::explore_min_depth(node.left.as_ref().map(Rc::clone), depth + 1, min_depth);
            Self::explore_min_depth(node.right.as_ref().map(Rc::clone), depth + 1, min_depth);
            return;
        }

        let Some(min_depth) = min_depth else {
            *min_depth = Some(depth);
            return;
        };

        if *min_depth > depth {
            *min_depth = depth;
        }
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
    fn step1_dfs_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step1_dfs_test() {
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
