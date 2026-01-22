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
  コメント集、他の人のコードを読んで考えたこと
  https://github.com/goto-untrapped/Arai60/pull/54#discussion_r1759975142
    > left, rightよりsmaller, largerの分かりやすいかと思いました。
  - なるほど。vよりも小さい値を持つノードで構成されるサブツリー、大きい値を持つノードで構成されるサブツリーに分けるので確かにそうだなと思った。
  二分探索木において、左は小さい、右は大きいという前提があるので自分はleft,rightにしているのだなと思った。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/41/changes#r1710591176
  - 型エイリアスの使い方や、そもそも型とはどのような使い方をするものなのかといったやりとりがされていて勉強になる。
  ここの議論と少しずれるが自分はRc<RefCell<TreeNode>>型のインスタンスを生成するときに毎回書くのが面倒なので、ヘルパーメソッドとしてbuild_nodeを実装した。
  type Tree = Rc<RefCell<TreeNode>>;　としてシグネチャで利用しても良いかと思ったが、TreeよりはTreeNodeの方が意味が近く型エイリアスにすることで意味がおかしくなりそうだったので止めた。
  この点についてよりよい方法がありそうなのでGPT-5.2に聞いてみたところ、参照であることを示す 型名+Refといった型エイリアス命名パターンがよく用いられているとのことだった。
  今回の例では type TreeNodeRef = Rc<RefCell<TreeNode>>; 又は type TreeNodeHandle = Rc<RefCell<TreeNode>> とするのがRustの文化的に一般的そうだった。
  reqwestクレートでは参照でラップされる側の型名にRefとつけていた。おそらくnewtypeパターンと呼ばれるもの。（reqwestクレートのソースコードを見たときの記憶が少し残っていて思い出した。）
  https://docs.rs/reqwest/latest/src/reqwest/async_impl/client.rs.html#94

  改善する時に考えたこと
  - 変数命名について left,right -> smaller,larger にする。小さいまとまりと大きいまとまりに分けているためこっちのほうが意味が近いと思った。
  - 型エイリアスを活用する
  - TreeNode同士の比較をOrdトレイトを実装して、シンプルに比較できるようにする。

  所感
  - すっきりと書けた気はするものの、Ordトレイトの実装内で毎回O(n)となるようなノードの数え上げを暗黙的に行っているのが気になった。
  比較演算子による比較(Ord impl)という低いレベルの操作でノードの数え上げ処理を暗黙的に呼び出しており、利用者側が意図せずコストの高い処理を呼び出してしまうという見方もできる。
  ただ、ノードの個数をTreeNodeのフィールドとして持つことでキャッシュすれば良いかというとそうでもなくて、ノードの追加・削除時に必ずノード個数（キャッシュ）の更新を行う必要がある。
  この場合、ノード追加・削除処理をTreeNodeが公開するのAPI経由でしか行えないように実装しないと、キャッシュしたノード個数が不正な値になりえるのでやっかいなバグになりそう。
  TreeNodeのインスタンス同士の比較条件をOrd実装で表現することによって、「ツリーを構成するノードの個数による比較をする。等しい場合は根の値で比較する。」というドメイン知識をカプセル化できているのは良い点だと思った。
*/

use std::{cell::RefCell, cmp::Ordering, collections::VecDeque, rc::Rc};

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
        let mut nodes = VecDeque::from_iter([
            self.left.as_ref().map(Rc::clone),
            self.right.as_ref().map(Rc::clone),
        ]);

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                continue;
            };

            nodes_count += 1;
            nodes.push_back(node.borrow().left.as_ref().map(Rc::clone));
            nodes.push_back(node.borrow().right.as_ref().map(Rc::clone));
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

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct Solution {}
impl Solution {
    pub fn split_b_s_t(root: TreeNodeRef, v: u32) -> TreeNodeRef {
        let (smaller_root, larger_root) = match Self::split_tree(Some(root), v) {
            (Some(smaller_root), None) => return smaller_root,
            (None, Some(larger_root)) => return larger_root,
            (Some(smaller_root), Some(larger_root)) => (smaller_root, larger_root),
            _ => unreachable!(),
        };

        if smaller_root < larger_root {
            return larger_root;
        };
        smaller_root
    }

    fn split_tree(node: Option<TreeNodeRef>, v: u32) -> (Option<TreeNodeRef>, Option<TreeNodeRef>) {
        match node {
            None => (None, None),
            Some(node) => {
                let node_val = node.borrow().val;

                if node_val <= v {
                    let right_node = node.borrow_mut().right.take();

                    let (smaller_root, larger_root) = Self::split_tree(right_node, v);
                    node.borrow_mut().right = smaller_root;

                    (Some(node), larger_root)
                } else {
                    let left_node = node.borrow_mut().left.take();

                    let (smaller_root, larger_root) = Self::split_tree(left_node, v);
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
    fn step2_split_bst_test_1() {
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
    fn step2_split_bst_test_2() {
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
    fn step2_to_nodes_test() {
        let node_values = vec![Some(5), None, Some(7), Some(6)];
        let root = to_binary_search_tree(&node_values);
        assert_eq!(to_node_values(root), node_values);
    }

    #[test]
    fn step2_to_binary_search_tree_test() {
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
    fn step2_count_nodes_test() {
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
