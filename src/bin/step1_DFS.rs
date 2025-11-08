// Step1_DFS
// 目的: 再帰によるDFSを実装してみる

/*
  問題の理解
  - 二分木の根rootと整数target_sumが与えられる。
  二分木の根からリーフ（子を持たないノード）までに出現したノードの値の合計値がtarget_sumに等しい経路があればTrueを返す。
  見つからなければFalseを返す。

  何を考えて解いていたか
  - 再帰によるDFSだと帰りがけのなるので、BFSで行った行きがけ（根からリーフまで階層を下っていく）とは違う。
  - 再帰関数の設計は基本ケースと再帰ケースからなる。今回の問題における基本ケースと再帰ケースを考えてみる。
    - 基本ケース
      - リーフ（子ノードを持たない）であれば終了することは分かる。リーフノードの時点でtarget_sumとの差が0かどうかの結果をboolで返せばよいか？
    - 再帰ケース
      - left,rightにノードをそれぞれ再帰処理で実行する。target_sumにはnode.valを引いた値を渡す。
      - left又はrightどちらかがTrueであればtarget_sumに一致するパスを見つけたと判定する。
    BFSと違って早期リターンできず、常に全ての根まで見に行く必要がある気がして非効率な感覚がある。

  正解してから気づいたこと
  - 一発で通るコードを書けたし、再帰を理解できていることを確認するために基本ケースと再帰ケースに分けて言語化してからコーディングしたが、まだおぼつかない感覚(若干の不安さ)が消えない。
    - 繰り返し練習して、自信をつけるしかなさそう
  - step1.rsのBFSと違って早期リターンができないので、こちらのDFSによる再帰の方が非効率だという感覚がある。実際どうなのか一応ChatGPT(GPT-5)に聞いてみる。
   - 結論としては単純にどちらの解法が優れているということはない。与えられる木の状態に依存するため。また、DFSの場合でも || による判定で左辺がTrueであれば右辺は評価されないので早期リターンになっている。(左側で見つければ右側は見に行かない。)
     - BFS（幅優先探索）
       - 浅い位置の解を見つければ良いかつ極端に木の幅が広くないときに利用すると良さそう。木の幅が極端に広いと層ごとにキューに入れるので空間計算量的に不利になる。
       - 完全二分木は幅が広くなり、深さがlog Nになる。今回の問題ではbinary treeとしか問題文にないので、幅が広いのか、高さが深いのか分からない。
     - DFS（深さ優先探索）
       - 幅が広い完全二分木かつ全探索が必要な場合に空間計算量が有利になる。一度に利用する補助空間計算量が木の深さO(log N)になるため。
       - 直列になるような極端に深い二分木が入力として与えられると再帰によるDFSはスタックオーバーフローし得るので注意が必要。
  - 今回の問題では、
    - ノードの上限数が5000と事前に分かっている
    - 再帰によるDFSの方がシンプルに記述できる
    - 問題文から根から最短の位置に答えがあるとは読み取れない
  以上の理由から、再帰によるDFSが良さそう。
  iterable(反復的)なDFSも実装しておく。
*/

use std::{cell::RefCell, rc::Rc};

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(node) = root else {
            return false;
        };
        let node = node.borrow();
        let is_leaf = node.left.is_none() && node.right.is_none();
        let target_sum = target_sum - node.val;

        if is_leaf {
            return target_sum == 0;
        };

        let (left_node, right_node) = (
            node.left.as_ref().map(Rc::clone),
            node.right.as_ref().map(Rc::clone),
        );

        Self::has_path_sum(left_node, target_sum) || Self::has_path_sum(right_node, target_sum)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step1_dfs_has_target_sum_test() {
        let node_values = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 22), true);

        let node_values = vec![Some(1)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 1), true);

        let node_values = vec![Some(1), Some(0)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 1), true);

        let node_values = vec![Some(-1), Some(1)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), true);

        let node_values = vec![Some(-1), None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, -1), true);
    }

    #[test]
    fn step1_dfs_has_not_target_sum_test() {
        let node_values = vec![Some(1), Some(2), Some(3)];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 5), false);

        let node_values = vec![None, None, None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);

        let node_values = vec![];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);

        let node_values = vec![None];
        let root = vec_to_binary_tree(&node_values);
        assert_eq!(Solution::has_path_sum(root, 0), false);
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
    fn step1_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
