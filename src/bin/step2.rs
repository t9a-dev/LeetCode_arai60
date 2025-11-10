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
  - 次に見るノードをキューではなく変数を上書きすることで管理している。個人的にはキューを使った出し入れの方が自然かなと思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/27/files#diff-d9b414492f03449c833e59a37973e7a5ce88ad5d813d7f797ae3d552d1389e1e

  - 自分はノードの値を入れるときにキューの先頭から入れるか、後ろから入れるかで調整した。
  この実装では常に左優先で入れいておいて、必要なときにreverse()することで実現している。実装例を見るまで思いつかなかった。
  一度配列に入れておいてreverse()するよりは、配列に入れる時点で順番が分かっているのでキューによる先頭又は末尾から追加していく方が自然だなと個人的には感じた。
  自分の実装だとVecDequeを利用していることが原因で、値を返すときに型変換が必要になっていて冗長な感じはあるが。
  この点について考えていて思ったが、単純に階層ごとにノードの値を集めてから、最後に全走査して配列のインデックスが偶数かどうかでreverseしてしまうのもありかなどと思った。
  最後に配列のインデックスをreverse()するところは、to_zigzag_orderメソッドとかにしてしまえば、階層下りながらキューの先頭に入れるか後ろから入れるか考える必要がなくなりは少し読みやすくはなるかな？という感じ。
  階層ごとにノードを集めるだけなのか、ノードを集めながらその階層が奇数が偶数かを判定してさらにキューの先頭又は末尾から突っ込むのか考えるよりは単純になるという感覚。
  https://github.com/olsen-blue/Arai60/pull/27/files#diff-8408720477e1fa6f21dd1fc886a99aa3a937252dddfe9a431516bbb8e258e6fdR72

  - 上記で考えていたことが言及されていた。
  https://github.com/5103246/LeetCode_Arai60/pull/26#discussion_r2444417265

  - is_left_to_rightは明示的でかなり分かりやすい変数名だと思った。
  https://github.com/nanae772/leetcode-arai60/pull/27/files#diff-7f81785a9b5cd8d28036603b485a39b2d30b84f51672a32f26c89b6c4199d709R8

  改善する時に考えたこと
  - BFSによる探索中にzigzagにするのではなく、戻り値を返す直前でまとめて配列のインデックスを見ながらzigzagにする方法で書いてみる。
    BFSによる左側優先探索によるノードの値を集めて配列に詰める作業とzigzagにする作業を別にすることで読んだときのワーキングメモリを削減できるのでは？という狙い。

  所感
  - zigzag並びへの変更処理をメソッドへ切り出したことで何をしているのか理解しやすくなっていると感じた。記述量の観点からは数行増える程度で済んでいる。
  読み手のことを考えるならこちらかなと思った。
*/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
pub struct Solution {}
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };
        let mut level_order_nodes: Vec<Vec<_>> = Vec::new();
        let mut frontiers = VecDeque::new();

        frontiers.push_back((root, 0));
        while let Some((node, level)) = frontiers.pop_front() {
            let node = node.borrow();

            match level_order_nodes.get_mut(level) {
                Some(nodes) => nodes.push(node.val),
                None => level_order_nodes.push(vec![node.val]),
            }

            let (left_node, right_node) = (
                node.left.as_ref().map(Rc::clone),
                node.right.as_ref().map(Rc::clone),
            );

            if let Some(node) = left_node {
                frontiers.push_back((node, level + 1));
            }
            if let Some(node) = right_node {
                frontiers.push_back((node, level + 1));
            }
        }

        Self::to_zigzag_order(level_order_nodes)
    }

    fn to_zigzag_order(mut level_order_nodes: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for (i, nodes) in level_order_nodes.iter_mut().enumerate() {
            let is_left_to_right_order = i % 2 == 0;

            if !is_left_to_right_order {
                let nodes: &mut Vec<_> = nodes.as_mut();
                nodes.reverse();
            }
        }

        level_order_nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let root = vec_to_binary_tree(&vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expect = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![
            Some(3),
            Some(9),
            Some(20),
            Some(4),
            None,
            None,
            Some(7),
        ]);
        let expect = vec![vec![3], vec![20, 9], vec![4, 7]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![Some(3)]);
        let expect = vec![vec![3]];
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![None]);
        let expect: Vec<Vec<_>> = Vec::new();
        assert_eq!(Solution::zigzag_level_order(root), expect);

        let root = vec_to_binary_tree(&vec![]);
        let expect: Vec<Vec<_>> = Vec::new();
        assert_eq!(Solution::zigzag_level_order(root), expect);
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
