// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 2つの二分木が与えられるので、これらをマージして1つの二分木にして返す。
  マージルールは以下
  - 重なり合う位置のノードの値は合計する
  - 重なり合うノードが無いノードはそのまま新しいノードとして利用する

  何がわからなかったか
  - root1,root2両方のノードが存在する場合にのみpush_backしている理由
  片方しかノード存在しない場合は、そのノード以降でマージする必要が無いため。

  何を考えて解いていたか
  - 入力制限はノード数2000なのと、構造体の持つデータも大きくなく参照を取り回すだけなので大丈夫だろう。
  - BFSで2つのノードを同じ階層毎に扱いつつ、そのままマージ後のノードとして同じ階層のノードを作る感じ。
  実装する手が止まったので解答を見た。

  正解してから気づいたこと
  - 似たようなパターンのコードが多いので再帰で書くとすっきりと書けそうなパターンだと思った。step1_DFS.rsで再帰処理も実装する。
*/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
        let Some(root1) = root1 else {
            return root2;
        };
        let Some(root2) = root2 else {
            return Some(root1);
        };

        let merged = Rc::new(RefCell::new(TreeNode::new(
            root1.borrow().val + root2.borrow().val,
        )));
        let mut frontiers = VecDeque::new();

        frontiers.push_back((Rc::clone(&root1), Rc::clone(&root2), Rc::clone(&merged)));
        while let Some((node1, node2, merged_node)) = frontiers.pop_front() {
            let (node1, node2, mut merged_node) =
                (node1.borrow(), node2.borrow(), merged_node.borrow_mut());

            match (
                node1.left.as_ref().map(Rc::clone),
                node2.left.as_ref().map(Rc::clone),
            ) {
                (Some(left1), Some(left2)) => {
                    let merged_left = Rc::new(RefCell::new(TreeNode::new(
                        left1.borrow().val + left2.borrow().val,
                    )));
                    merged_node.left = Some(Rc::clone(&merged_left));
                    frontiers.push_back((left1, left2, merged_left));
                }
                (Some(left1), None) => merged_node.left = Some(left1),
                (None, Some(left2)) => merged_node.left = Some(left2),
                _ => (),
            }

            match (
                node1.right.as_ref().map(Rc::clone),
                node2.right.as_ref().map(Rc::clone),
            ) {
                (Some(right1), Some(right2)) => {
                    let merged_right = Rc::new(RefCell::new(TreeNode::new(
                        right1.borrow().val + right2.borrow().val,
                    )));
                    merged_node.right = Some(Rc::clone(&merged_right));
                    frontiers.push_back((right1, right2, merged_right));
                }
                (Some(right1), None) => merged_node.right = Some(right1),
                (None, Some(right2)) => merged_node.right = Some(right2),
                _ => (),
            }
        }

        Some(merged)
    }
}

#[cfg(test)]
mod tests {
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
    fn step1_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }

    #[test]
    fn step1_test() {
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
