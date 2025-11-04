// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  https://ja.wikipedia.org/wiki/%E4%BA%8C%E5%88%86%E6%9C%A8
  - 二分木(binary tree)の根(root)が与えられるので最大深さを返す。（二分木は「にぶんぎ」と読む）
  最大深さとは根のノードから最も離れたノードにたどり着いたときに出現したノードの数に等しい。
  4,5が最も深いノードで、最大深さは3になる。(1,2,4) or (1,2,5)
      1
     / \
     2  3
    / \
    4 5

  何がわからなかったか
  - テストコードを書こうと思ったが配列から、binary treeを構築する実装ができなかった。ChatGPTに聞いて写経した。
  - DFSを再帰処理で書こうとしたがうまくできなかったので、step1_DFS.rsで書いてみる。

  何を考えて解いていたか
  - 入力制約からツリーのノードの数は10^4とある。二分木なので直列にならず、log nくらいの最大深さになりそうな気がする。
    - 感覚的には再帰処理によるスタックでも全然大丈夫そう。
  - 二分木を再帰処理によるDFSで探索して深さをカウントする方法がすぐに思いつかないことに気づいたのでBFSによる実装にした。

  想定ユースケース
  - 二分木の深さを数えることが何に役立つのか現時点で全く思いつかない。

  正解してから気づいたこと
  - BFSで利用するキューの変数命名に迷った。
  これまで読んできた他の人のコードでfrontier(未開拓の分野的な意味)が候補として上がっていた記憶があったので、これから見るノードという意味でこれにした。
  - 最初に再帰処理によるDFSで解こうと思ったが、スムーズにできなかったので再帰処理が相変わらず苦手だなと思った。
  - Rcはcloneメソッドより、Rc::cloneを利用するべきだったが、node.borrow().left.map(|v| Rc::clone(&v))のように冗長な書き方しかできなかったのでnode.borrow().left.clone()呼び出しにしておいた。
  より良い書き方が分かっていないだけな可能性があるのでstep2で調べる。
  https://doc.rust-jp.rs/book-ja/ch15-04-rc.html#:~:text=Rc%3A%3Aclone%28%26a%29%E3%81%A7%E3%81%AF%E3%81%AA%E3%81%8F%E3%80%81a%2Eclone%28%29%E3%82%92%E5%91%BC%E3%81%B6%E3%81%93%E3%81%A8%E3%82%82%E3%81%A7%E3%81%8D%E3%81%BE%E3%81%99%E3%81%8C%E3%80%81Rust%E3%81%AE%E3%81%97%E3%81%8D%E3%81%9F%E3%82%8A%E3%81%AF%E3%80%81%E3%81%93%E3%81%AE%E5%A0%B4%E5%90%88Rc%3A%3Aclone%E3%82%92%E4%BD%BF%E3%81%86%E3%81%93%E3%81%A8%E3%81%A7%E3%81%99%E3%80%82%20Rc
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut frontier_nodes = VecDeque::new();

        frontier_nodes.push_back((root, 1));
        while let Some((node, depth)) = frontier_nodes.pop_front() {
            let Some(node) = node else {
                continue;
            };

            max_depth = depth;

            frontier_nodes.push_back((node.borrow().left.clone(), depth + 1));
            frontier_nodes.push_back((node.borrow().right.clone(), depth + 1));
        }

        max_depth
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
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 1);

        let node_values = vec![Some(3), None, Some(5)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 2);

        let node_values = vec![];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 0);

        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 3);
    }
}
