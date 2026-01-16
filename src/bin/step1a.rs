// step1a
// 目的: 別の実装方法(再帰->ループ)で練習する。

/*
  問題の理解
  - 二分探索木の根であるrootと整数vが与えられる。
  あるノードから見た時に左側のサブツリーにはノード自身が持つ値より小さい値を持つノードで構成される
  あるノードから見た時に右側のサブツリーにはノード自身が持つ値より大きい値を持つノードで構成される
  整数v以下のノードで構成されるツリーと整数vを超えるノードで構成されるツリーに分割する。
  分割したサブツリーうちノード数が多い方のサブツリーを返す。サブツリーの数が同じ場合はrootノードの値が大きい方のサブツリーを返す。

  step1からの変更点
  - count_nodesの再帰処理をループへ変更
  - split_treeの再帰処理をループへ変更
    - ループへの書き直しを行ったが、直列のになるような木構造を扱えないロジックが出来上がったので、GPT-5.2に聞きつつ修正した。
    dummy,tailポインタで操作対象のノードのポインタを管理する修正を行った。

  所感
    - split_b_s_tでノードの数が多いサブツリー（等しいときは根のノードの値が大きい方）を返しているが、比較の仕様をOrdトレイトといして実装して、TreeNode自体で比較できるようにするのも良さそう。
    ツリーを構成するノードの数が多い(等しいときはツリーの根の値が大きい)方を大きいサブツリーとするというドメイン知識をTreeNodeにカプセル化できるという感覚。
*/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    val: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    pub fn build_node(val: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    pub fn count_nodes(&self) -> usize {
        let mut node_counts = 1;
        let mut frontiers = VecDeque::from_iter([
            self.left.as_ref().map(Rc::clone),
            self.right.as_ref().map(Rc::clone),
        ]);

        while let Some(node) = frontiers.pop_front() {
            let Some(node) = node else {
                continue;
            };
            let node = node.borrow();

            node_counts += 1;
            frontiers.push_back(node.left.as_ref().map(Rc::clone));
            frontiers.push_back(node.right.as_ref().map(Rc::clone));
        }

        node_counts
    }

    fn new(val: u32) -> Self {
        Self {
            val,
            right: None,
            left: None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn split_b_s_t(root: Rc<RefCell<TreeNode>>, v: u32) -> Rc<RefCell<TreeNode>> {
        let (left_subtree, right_subtree) = Self::split_tree(root, v);
        let (left_subtree, right_subtree) = match (left_subtree, right_subtree) {
            (Some(l), None) => return l,
            (None, Some(r)) => return r,
            (Some(l), Some(r)) => (l, r),
            (None, None) => unreachable!(),
        };

        let (left_node_count, right_node_count) = (
            left_subtree.borrow().count_nodes(),
            right_subtree.borrow().count_nodes(),
        );
        if left_node_count < right_node_count {
            return right_subtree;
        } else if right_node_count < left_node_count {
            return left_subtree;
        };

        if left_subtree.borrow().val < right_subtree.borrow().val {
            return right_subtree;
        }

        left_subtree
    }

    fn split_tree(
        root: Rc<RefCell<TreeNode>>,
        v: u32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        let dummy_left_root = TreeNode::build_node(0);
        let dummy_right_root = TreeNode::build_node(0);
        let mut left_subtree_tail = Rc::clone(&dummy_left_root);
        let mut right_subtree_tail = Rc::clone(&dummy_right_root);
        let mut current_node_cursor = Some(root);

        while let Some(node) = current_node_cursor.as_ref().map(Rc::clone) {
            let node_val = node.borrow().val;

            if node_val <= v {
                let next_node_cursor = node.borrow_mut().right.take();

                left_subtree_tail.borrow_mut().right = Some(Rc::clone(&node));
                left_subtree_tail = node;

                current_node_cursor = next_node_cursor;
            } else {
                let next_cursor = node.borrow_mut().left.take();

                right_subtree_tail.borrow_mut().left = Some(Rc::clone(&node));
                right_subtree_tail = node;

                current_node_cursor = next_cursor;
            }
        }

        let (left_subtree, right_subtree) = (
            dummy_left_root.borrow().right.as_ref().map(Rc::clone),
            dummy_right_root.borrow().left.as_ref().map(Rc::clone),
        );

        (left_subtree, right_subtree)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn to_binary_search_tree(node_values: &Vec<Option<u32>>) -> Rc<RefCell<TreeNode>> {
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
    fn step1a_split_bst_test_1() {
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
        let picked = Solution::split_b_s_t(root, 2);
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
        let picked = Solution::split_b_s_t(root, 4);
        assert_eq!(picked.borrow().val, 4);
        assert_eq!(picked.borrow().count_nodes(), 4);
    }

    #[test]
    fn step1a_split_bst_test_2() {
        // BST: [5,2,7,1,3,6]
        // v=4: <=4 is {2,1,3}, >4 is {5,7,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = Solution::split_b_s_t(root, 4);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 3);

        // BST: [5,2,7,1,3,6]
        // v=1: <=1 is {1}, >1 is {5,2,7,#,3,6}
        let root =
            to_binary_search_tree(&vec![Some(5), Some(2), Some(7), Some(1), Some(3), Some(6)]);
        let picked = Solution::split_b_s_t(root, 1);
        assert_eq!(picked.borrow().val, 5);
        assert_eq!(picked.borrow().count_nodes(), 5);
        assert_eq!(
            to_node_values(picked),
            vec![Some(5), Some(2), Some(7), None, Some(3), Some(6)]
        );
    }

    #[test]
    fn step1a_to_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(to_node_values(root), node_values);
    }

    #[test]
    fn step1a_to_binary_search_tree_test() {
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
    fn step1a_count_nodes_test() {
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
