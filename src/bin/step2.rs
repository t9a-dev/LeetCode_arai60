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
  - step1_DFS.rsでDFS,BFSどちらが良いのか検討したが、同じような感じで迷っている人がいた。
  説明できるように整理しておくのは良いことだと思った。
  https://github.com/potrue/leetcode/pull/25/files#diff-0eaf8518dfee4676119c183e1e95579c78fe53bfffd4da19b1eea20e46d82f09R27

  - None値をスタックやキューに積んで、取り出した後に空かどうか判定することについて。自分も別の問題で似たような指摘を受けた覚えがある。
  push,popにも計算資源を消費するので、積む前に必要無いことが分かるなら積まない方が良いよねという考え方に同意できると思った。
  書いていて気付いたが、再帰呼び出しするときに記述量は増えるもののNoneの場合は関数呼出しをスキップできるなと思った。（再帰呼び出しで判定するのではなく、呼び出し前に判定して無駄な関数呼出しを減らす。）
  一度実装してみたが、条件分岐による記述量が多く再帰処理による記述のシンプルさが大幅に損なわれるので、無駄な関数呼出しは増えるものの機械に仕事をさせることの範囲内だと思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/25/files#r2126158749

  - 変数名に *_so_far とつけるのは初めて見た。「これまでの累積値」、「現在までの途中経過」というニュアンスらしい。あまりピンとこないので自分は使わないが語彙が増えて良かった。
  https://github.com/quinn-sasha/leetcode/pull/24/files#r2172911737

  - 自分は数え上げるような実装がうまく思いつかなかったのでtarget_sumを引く方向で実装した。
  コメントにもあるが問題文の文脈的にはノードの値を足し合わせていって確認するほうが自然ではあるなと思った。
  https://github.com/h1rosaka/arai60/pull/28#discussion_r2280599394

  - 自分はstep1のiterable実装でfrontiersとした。
  この問題における変数の使われ方やコードの長さを考えるとLIFOであること以上の情報はいらない気がするのでstackでも良いと思った。
  https://github.com/h1rosaka/arai60/pull/28#discussion_r2285795311

  - is_leafをメソッドとして定義しているコードを良く見かけた。
  個人的には一度しか記述しないのに関数呼出しにすることに忌避感を感じるので、変数名で意味を明示的にする方法としている。
  しかし、実務に寄ったクラス設計を考えると、is_leaf判定は複数回行われそうなのでメソッド化しても良いかなどと考えた。

  改善する時に考えたこと
  - 特になし
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
        }

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
    fn step2_has_target_sum_test() {
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
    fn step2_has_not_target_sum_test() {
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
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
