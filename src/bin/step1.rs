// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 二分探索木の根であるrootと整数vが与えられる。
  あるノードから見た時に左側のサブツリーにはノード自身が持つ値より小さい値を持つノードで構成される
  あるノードから見た時に右側のサブツリーにはノード自身が持つ値より大きい値を持つノードで構成される
  整数v以下のノードで構成されるツリーと整数vを超えるノードで構成されるツリーに分割する。
  分割したサブツリーうちノード数が多い方のサブツリーを返す。サブツリーの数が同じ場合はrootノードの値が大きい方のサブツリーを返す。

  何を考えて解いていたか
  - テストについて。
    - 採点システムがRustに対応していないので、テストコードの実装を行う。過去に二分探索木の問題を解いたときのヘルパーメソッドが流用できそうだが、折角なので練習として再度書いてみる。
      - 配列から二分探索木を構築するヘルパーメソッドを実装する必要がある。
      - 二分探索木から配列を構築するヘルパーメソッドを実装する必要がある。
  - 問題の解法について。
    - サブツリーに含まれるノードの数を数える必要がある。サブツリーのルートノードを入力としてノードの数を返す処理が必要。
      - もしかすると、サブツリーに分割する時点でどちらのサブツリーの方がノードの数が多いか分かる方法があるかも知れないが思いつかない。
    - vを境界にして分割する方法が思いつかない。
  手が止まったので解答をみる。

  何がわからなかったか
  - ノードを分割する手順。あるノードの値がv以下で、このノードの子の値がv以上のときにどういう手順でノードを入れ替えるのかがうまく言語化できない。

  解法の理解(GPT-5.2の出力コードを自分で理解しやすいようにリファクタリングした)
  - 概要
    - 整数vを境界として、二分探索木を2つの二分探索木に分割する。(node.val <= v, v < node.val)
  - 再帰の処理内容
    - base_case
      - ノードがNoneであれば、子ノードも存在しないので、(None,None)を返すことで、見るべきサブツリーが存在しないことを表現する。
    - recursive_case
      - v以下の値を持つノードから構成されるサブツリー(left_subtree)、vを超える値を持つノードから構成されるサブツリー(right_subtree)を作りたいという気持ち。
        - ノードの値がv以下のとき
          - 左の子ノードが存在するとき、v以下である値を持つことは確定しているので、未確定な右の子ノードを取り出してrecursive_caseに入る
            - 二分探索木なので、あるノードの左の子ノードは親ノードより小さい値を持つ
          - recursive_caseから得られたv以下の値から構成されるleft_subtreeをnode.rightに設定する。
        - ノードの値がvを超えるとき
          - 右の子ノードが存在するとき、vを超える値を持つことは確定しているので、未確定な左の子ノードを取り出してrecursive_caseに入る
            - 二分探索木なので、あるノードの右の子ノードは親ノードより大きい値を持つ
          - recursive_caseから得られたvを超える値から構成されるright_subtreeをnode.leftに設定する。

  所感
  - GPT-5.2のコードがかなり読みづらかったので、これを読みながら自分なりに読みやすくリファクタリングする作業が理解を進めるよい練習になった。
    - GPT-5.2のコードでは二分探索木の分割をしながら、ノードの数え上げも行っていたので読みづらかったが、プロコン文脈でパフォーマンスを取るならそっちの方が良いことは理解できる。
    一方で、自分の実装のほうが可読性が優れていると考える。ノードの数え上げと分割をそれぞれ分離できているため。
    - 時間計算量の観点では、ワンパスでできるかどうかなので、プロコン文脈でなければ自分の実装で良いと考えた。
    最適化するのであれば、ノードの数え上げ処理をループによる実装に書き換えて再帰処理による関数呼び出しのオーバーヘッドを削減する。
  - ループでの実装も練習する。
*/

use std::{cell::RefCell, rc::Rc};

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
        fn count_node(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            match node {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    1 + count_node(&node.left) + count_node(&node.right)
                }
            }
        }

        1 + count_node(&self.left) + count_node(&self.right)
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
        let (left_tree, right_tree) = Self::split_tree(Some(root), v);

        let (left_tree, right_tree) = match (left_tree, right_tree) {
            (Some(l), None) => return l,
            (None, Some(r)) => return r,
            (Some(l), Some(r)) => (l, r),
            (None, None) => unreachable!(),
        };

        let (left_node_count, right_node_count) = (
            left_tree.borrow().count_nodes(),
            right_tree.borrow().count_nodes(),
        );
        if left_node_count < right_node_count {
            return right_tree;
        } else if right_node_count < left_node_count {
            return left_tree;
        };

        if left_tree.borrow().val < right_tree.borrow().val {
            return right_tree;
        }

        left_tree
    }

    fn split_tree(
        node: Option<Rc<RefCell<TreeNode>>>,
        v: u32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (None, None),
            Some(node) => {
                let node_val = node.borrow().val;

                if node_val <= v {
                    let right_node = node.borrow_mut().right.take();
                    let (left_subtree, right_subtree) = Self::split_tree(right_node, v);

                    node.borrow_mut().right = left_subtree;

                    (Some(node), right_subtree)
                } else {
                    let left_node = node.borrow_mut().left.take();
                    let (left_subtree, right_subtree) = Self::split_tree(left_node, v);

                    node.borrow_mut().left = right_subtree;

                    (left_subtree, Some(node))
                }
            }
        }
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
    fn step1_split_bst_test_1() {
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
    fn step1_split_bst_test_2() {
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
    fn step1_to_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(to_node_values(root), node_values);
    }

    #[test]
    fn step1_to_binary_search_tree_test() {
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
    fn step1_count_nodes_test() {
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
