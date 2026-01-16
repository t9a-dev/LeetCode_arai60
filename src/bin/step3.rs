// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nodes.len()
  時間計算量: O(n)
  空間計算量: O(n)
*/

/*
  構造体の定義は除き、メソッドの中身は実装したときの時間
  1回目: 10分15秒
  2回目: 11分3秒
  3回目: 8分14秒
*/

/*
  所感
  - ノードの数え上げ処理(count_nodes)でVecDequeを使う必要はないことに気付いたのでVecにした。
*/

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    val: u32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}
impl TreeNode {
    pub fn build_node(val: u32) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    fn count_nodes(&self) -> usize {
        let mut nodes_count = 1;
        let mut nodes = Vec::from_iter([
            self.left.as_ref().map(Rc::clone),
            self.right.as_ref().map(Rc::clone),
        ]);

        while let Some(node) = nodes.pop() {
            let Some(node) = node else {
                continue;
            };

            nodes_count += 1;
            nodes.push(node.borrow().left.as_ref().map(Rc::clone));
            nodes.push(node.borrow().right.as_ref().map(Rc::clone));
        }

        nodes_count
    }

    fn new(val: u32) -> Self {
        Self {
            val,
            right: None,
            left: None,
        }
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count_nodes()
            .cmp(&other.count_nodes())
            .then(self.val.cmp(&other.val))
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct Solution {}
impl Solution {
    pub fn split_binary_search_tree(root: TreeNodeRef, v: u32) -> TreeNodeRef {
        let (smaller_root, larger_root) = match Self::split_binary_search_tree_helper(Some(root), v)
        {
            (Some(smaller_root), None) => return smaller_root,
            (None, Some(larger_root)) => return larger_root,
            (Some(smaller_root), Some(larger_root)) => (smaller_root, larger_root),
            (None, None) => unreachable!(),
        };

        if smaller_root < larger_root {
            return larger_root;
        }
        smaller_root
    }

    fn split_binary_search_tree_helper(
        node: Option<TreeNodeRef>,
        v: u32,
    ) -> (Option<TreeNodeRef>, Option<TreeNodeRef>) {
        match node {
            None => (None, None),
            Some(node) => {
                let node_val = node.borrow().val;

                if node_val <= v {
                    let right_node = node.borrow_mut().right.take();
                    let (smaller_root, larger_root) =
                        Self::split_binary_search_tree_helper(right_node, v);

                    node.borrow_mut().right = smaller_root;

                    (Some(node), larger_root)
                } else {
                    let left_node = node.borrow_mut().left.take();
                    let (smaller_root, larger_root) =
                        Self::split_binary_search_tree_helper(left_node, v);

                    node.borrow_mut().left = larger_root;

                    (smaller_root, Some(node))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn to_binary_search_tree(node_values: &Vec<Option<u32>>) -> TreeNodeRef {
        let get_node_value = |i: usize| -> Option<u32> {
            let Some(node_value) = node_values.get(i) else {
                return None;
            };
            *node_value
        };
        let Some(root_node_value) = get_node_value(0) else {
            panic!("require root node value");
        };
        let root = TreeNode::build_node(root_node_value);
        let mut nodes = VecDeque::from_iter([Rc::clone(&root)]);

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            let mut node = node.borrow_mut();

            if let Some(left_node_value) = get_node_value(i) {
                let left_node = TreeNode::build_node(left_node_value);
                node.left = Some(Rc::clone(&left_node));
                nodes.push_back(left_node);
            }
            i += 1;

            if let Some(right_node_value) = get_node_value(i) {
                let right_node = TreeNode::build_node(right_node_value);
                node.right = Some(Rc::clone(&right_node));
                nodes.push_back(right_node);
            }
            i += 1;
        }

        root
    }

    fn to_node_values(root: Rc<RefCell<TreeNode>>) -> Vec<Option<u32>> {
        let mut nodes = VecDeque::from_iter([Some(root)]);
        let mut node_values = vec![];

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            let node = node.borrow();
            node_values.push(Some(node.val));
            nodes.push_back(node.left.as_ref().map(Rc::clone));
            nodes.push_back(node.right.as_ref().map(Rc::clone));
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn step3_split_bst_test_1() {
        // BST: [4,2,6,1,3,5,7]
        // v=2: <=2 is {2,1}, >2 is {4,3,6,5,7}
        let root = to_binary_search_tree(&vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        let picked = Solution::split_binary_search_tree(root, 2);
        assert_eq!(picked.borrow().val, 4);
        assert_eq!(picked.borrow().count_nodes(), 5);

        // v=4: <=4 is {4,2,1,3}, >4 is {6,5,7}
        let root = to_binary_search_tree(&vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        let picked = Solution::split_binary_search_tree(root, 4);
        assert_eq!(picked.borrow().val, 4);
        assert_eq!(picked.borrow().count_nodes(), 4);
    }

    #[test]
    fn step3_split_bst_test_2() {
        // BST: [5,2,7,1,3,6]
        // v=4: <=4 is {2,1,3}, >4 is {5,7,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = Solution::split_binary_search_tree(root, 4);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 3);

        // BST: [5,2,7,1,3,6]
        // v=1: <=1 is {1}, >1 is {5,2,7,#,3,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = Solution::split_binary_search_tree(root, 1);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 5);
        assert_eq!(
            to_node_values(picked),
            vec![Some(5), Some(2), Some(7), None, Some(3), Some(6)]
        );
    }

    #[test]
    fn step3_to_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(to_node_values(root), node_values);
    }

    #[test]
    fn step3_to_binary_search_tree_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let actual_root = to_binary_search_tree(&node_values);

        let expect_root = TreeNode::build_node(5);
        expect_root.borrow_mut().right = Some(TreeNode::build_node(7));
        expect_root
            .borrow_mut()
            .right
            .as_ref()
            .map(Rc::clone)
            .unwrap()
            .borrow_mut()
            .left = Some(TreeNode::build_node(6));

        assert_eq!(actual_root, expect_root);
    }

    #[test]
    fn step3_count_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 3);

        let node_values = vec![Some(5), None, Some(7)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 2);

        let node_values = vec![Some(5)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(root.borrow().count_nodes(), 1);
    }
}
