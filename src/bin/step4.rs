// Step4
// 目的: 少し時間が余ったのでこれまでもらったレビューや、コメント集を調べる。

/*
  以前やった問題のレビューで再帰関数をiterativeにやってみるのも良いとのことだったのでやる。
  https://github.com/t9a-dev/LeetCode_arai60/pull/19#discussion_r2486185608

  - 「末尾呼び出し」と「末尾呼び出しの最適化」
  https://github.com/ichika0615/arai60/pull/7#discussion_r1877624318
  https://www.fos.kuis.kyoto-u.ac.jp/~igarashi/class/pl/06-rec-iter.html#fnref2
    - 「末尾呼び出し」 関数の返り値の位置にある関数呼出しのこと。
    - 「末尾呼び出しの最適化」 末尾呼び出しをコンパイラやランタイムが検出して、スタックを再利用して実行すること。スタックオーバーフローが起きない。
      - Rust言語は「末尾呼び出しの最適化」が行われないので、スタックオーバーフローを回避したい場合は明示的に繰り返し文にコードを書き換える必要がある。
  - 末尾呼び出しがコンパイラによって最適化され、アセンブリにおいてcallによる関数呼出し部分がjmp命令によってループ文になっている。
  （教育用のアセンブリしか読んだことが無いので雰囲気で読んでいる。）
  https://discord.com/channels/1084280443945353267/1235829049511903273/1236314277305122868

  - 再帰処理にするかどうかの簡単な判断基準
  step1でキューを利用したBFSの実装を行おうとして考えて手が止まったので、この時点でBFSによる再帰処理の方に発想を転換できればよかったなと思った。
  https://github.com/irohafternoon/LeetCode/pull/6#discussion_r2014558537
  > ... なので、単純なループに近いときには再帰でわざわざ書くことはあまりないとは思います。ただ結構考えるときには使っています。

  - 再帰を機械的に書き換えられるようになれば、理解できたと思って良さそう。
  https://discord.com/channels/1084280443945353267/1235829049511903273/1238532375240249494

  DFSを再帰ではない実装を写経しておく(自分で実装しようとしたが手が止まってしまった。)
  行きがけ順(preorder traversal)処理になっている。
  上から順にマージして子のノードを作っていくイメージなので、処理順序と名称に違和感を感じない。

  自分はstep1_DFS.rsで実装した再帰処理の帰りがけ(postorder traversal)を見ると混乱するのが分かった。
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

        let merged_root = Rc::new(RefCell::new(TreeNode::new(
            node1.borrow().val + node2.borrow().val,
        )));
        let mut stack = Vec::new();
        stack.push((
            Rc::clone(&node1),
            Rc::clone(&node2),
            Rc::clone(&merged_root),
        ));

        while let Some((node1, node2, merged_node)) = stack.pop() {
            let (node1, node2) = (node1.borrow(), node2.borrow());

            match (&node1.left, &node2.left) {
                (Some(left1), Some(left2)) => {
                    let child = Rc::new(RefCell::new(TreeNode::new(
                        left1.borrow().val + left2.borrow().val,
                    )));
                    merged_node.borrow_mut().left = Some(Rc::clone(&child));
                    stack.push((Rc::clone(&left1), Rc::clone(&left2), Rc::clone(&child)));
                }
                (Some(left1), None) => merged_node.borrow_mut().left = Some(Rc::clone(&left1)),
                (None, Some(left2)) => merged_node.borrow_mut().left = Some(Rc::clone(&left2)),
                _ => (),
            }

            match (&node1.right, &node2.right) {
                (Some(right1), Some(right2)) => {
                    let child = Rc::new(RefCell::new(TreeNode::new(
                        right1.borrow().val + right2.borrow().val,
                    )));
                    merged_node.borrow_mut().right = Some(Rc::clone(&child));
                    stack.push((Rc::clone(&right1), Rc::clone(&right2), Rc::clone(&child)));
                }
                (Some(right1), None) => merged_node.borrow_mut().right = Some(Rc::clone(&right1)),
                (None, Some(right2)) => merged_node.borrow_mut().right = Some(Rc::clone(&right2)),
                _ => (),
            }
        }

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
    fn step4_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }

    #[test]
    fn step4_test() {
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

        let root1 = vec_to_binary_tree(&vec![None]);
        let root2 = vec_to_binary_tree(&vec![]);
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);

        let root1 = vec_to_binary_tree(&vec![]);
        let root2 = vec_to_binary_tree(&vec![]);
        let expect = vec_to_binary_tree(&vec![]);
        assert_eq!(Solution::merge_trees(root1, root2), expect);
    }
}
