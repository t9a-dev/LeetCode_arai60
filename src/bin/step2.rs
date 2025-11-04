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
  - if文を減らすときの考え方、問題の捉え方について。
  Leet Codeで正解のコードを見る前は問題を複雑に捉えていたなと思うことがよくある。
  https://github.com/garunitule/coding_practice/pull/21/files#r2202868783

  - 単純に根から底まで深さを見に行くだけでなく、下から上に数え上げるような解法が実装されている。
  解法ののバリエーションとしてこれくらい自由にコードを変形できるのは良いなと思った。
  https://github.com/nanae772/leetcode-arai60/pull/21/files

  > 「... 複数の指標の中で速度が重視されることは実はあまりありません。」
  https://github.com/docto-rin/leetcode/pull/20/files#r2429390509
  日常的に書くコードではよりシンプルで自然に理解できるコードかという指標が重視されるという感覚はある。
  ソフトウェアエンジニアとしての常識に収まる範囲でシンプルで自然なコードを書けることが重要であると理解した。

  - LeetCode採点システムの実行速度はかなりばらつきがあるので、気になった処理の実行速度を手元で測定するしているのが良いなと思った。
  https://github.com/docto-rin/leetcode/pull/20/files#diff-4888d90f21aa6972ed6808284b2125b5a2c17a69b8e80975819c489b26a80330R87

  - このコードの通りBFSで利用するキューはメソッド内で一つしか登場しないのでfrontiersが必要十分だなと思った。
  https://github.com/docto-rin/leetcode/pull/20/files#diff-4888d90f21aa6972ed6808284b2125b5a2c17a69b8e80975819c489b26a80330R152

  改善する時に考えたこと
  - Rc(Reference Counter)型は.clone()ではなく、公式ドキュメントで推奨されているRc::clone(&self)によるクローン（参照カウントのインクリメント）とする。
  https://doc.rust-jp.rs/book-ja/ch15-04-rc.html#:~:text=Rc%3A%3Aclone%28%26a%29%E3%81%A7%E3%81%AF%E3%81%AA%E3%81%8F%E3%80%81a%2Eclone%28%29%E3%82%92%E5%91%BC%E3%81%B6%E3%81%93%E3%81%A8%E3%82%82%E3%81%A7%E3%81%8D%E3%81%BE%E3%81%99%E3%81%8C%E3%80%81Rust%E3%81%AE%E3%81%97%E3%81%8D%E3%81%9F%E3%82%8A%E3%81%AF%E3%80%81%E3%81%93%E3%81%AE%E5%A0%B4%E5%90%88Rc%3A%3Aclone%E3%82%92%E4%BD%BF%E3%81%86%E3%81%93%E3%81%A8%E3%81%A7%E3%81%99%E3%80%82%20Rc

  - 変数名frontier_nodes -> frontiers　メソッド内でキューは一つしか登場せず、これから見ていくノードを入れておくという意味で未開拓であるという意味がちょうどよいと思ったので。
  - LeetCodeのテンプレート上はTreeNodeの型がOption<Rc<RefCell<TreeNode>>>となっているが、この問題では内部可変性(RefCell)が必要ないのでOption<Box<TreeNode>>で行けるか気になったのでstep2_box_type.rsで試す。
  可変参照を使っていないので、内部可変性もいらないだろうという感覚。参照(スマートポインタ)のみで取り回せそう。
  - RefCellのborrow()をカジュアルに呼び出していたが、borrow(),borrow_mut()は連続で呼び出すとコンパイルエラーにならず実行時にパニックするのでかなり危ないと思った。(Rust書いてるとコンパイル通れば大体大丈夫と思いがちなので)
  テストコードで上記の操作を行っている実行パスを通れば検出できるが、実行時パニックは思ったより危ないと感じた。
  https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow
  try_borrow()で安全に取得できるので、borrow()はunwrap()に近いと思った。（操作が失敗するとパニックする）
  しかし、try_borrow()で失敗したときに無視するくらいしかできなさそうな感じはある。
  コードのインラインコメントで記載しているshadowing（再束縛）で緩和できる。
  ちなみに、borrow(),borrow_mut()が同時に呼ばれることが考えられる並列実行時にはRc<RefCell<T>> ではなくスレッドセーフな Arc<RwLock<T>> or Arc<Mutex<T>>を使う。
  https://doc.rust-jp.rs/book-ja/ch16-03-shared-state.html
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
        let mut frontiers = VecDeque::new();

        frontiers.push_back((root, 1));
        while let Some((node, depth)) = frontiers.pop_front() {
            let Some(node) = node else {
                continue;
            };

            max_depth = depth;

            // shadowing(再束縛)
            // https://doc.rust-jp.rs/rust-by-example-ja/variable_bindings/scope.html
            // ここでnodeを再定義することで、以降node.borrow(),node.borrow_mut()できなくなり二重borrow()によるpanicを緩和。
            // https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow:~:text=An%20example%20of%20panic
            let node = node.borrow();
            frontiers.push_back((node.left.as_ref().map(Rc::clone), depth + 1));
            frontiers.push_back((node.right.as_ref().map(Rc::clone), depth + 1));
        }

        max_depth
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
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step2_test() {
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
