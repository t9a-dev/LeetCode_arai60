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
  - インプレイスで重複削除を行う実装で内部可変性は必要なくてBox<T>型で可能。
  https://leetcode.com/problems/remove-duplicates-from-sorted-list/solutions/7028635/simple-iterative-rust-solution-0-ms/

  - 再帰処理は思いつかなかった。
  今回の入力ではスタックオーバーフローにならないが、個人的には再帰処理はスタックオーバーフローを気にする必要があるのであまり採用しないかなと思った。
  https://github.com/docto-rin/leetcode/pull/3/files#diff-9913e158f560d8811786bd686db2c5c97d7fa3e3dd69d4cadf990224a7ceaef7R133

  改善する時に考えたこと
  - 入力の型の制約でインプレイスによる重複削除ができないと勘違いしていた。空間計算量O(1)になる形で実装してみる。
  インプレイスでの実装を行うにあたり、コンパイラとの所有権バトルがかなりきつかったが、こういった機会が無いと理解が進まないので良い経験になった。
    - Option<T>からはtake()メソッドで所有権をムーブして元の場所にNoneが入る。
    - シングルスレッド実行であれば可変参照は普通に使えるので、なんでもかんでもRc<RefCell<T>>といった内部可変性をもたせる必要はない。
  - LeetCodeのメソッド定義でhead引数がmutableになっていないので、破壊的な変更を加えて良いか迷った。
  メソッド名がdelete_duplicateなので破壊的な変更を加えるのは予想できそうなので、LeetCode側でheadはmutableにしておいてほしいと思った。
  - while let Someでネストが深くなったので、重複判定の箇所の条件を反転して一番ネストが深い部分ではbreakのみ行うようにして可読性を上げた。
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {
            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }
                node.next = next_node.next.take();
            }
            current = node.next.as_mut();
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list_node(values: &Vec<i32>) -> Option<Box<ListNode>> {
        values.into_iter().rev().fold(None, |child, v| {
            let mut parent = Box::new(ListNode::new(*v));
            parent.next = child;
            Some(parent)
        })
    }

    fn list_node_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut next_node = head;

        while let Some(node) = next_node {
            out.push(node.val);

            next_node = &node.next;
        }

        out
    }

    #[test]
    fn step2_test() {
        let source_vec = vec![1, 1, 2];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1, 2]
        );

        let source_vec = vec![1, 1, 2, 3, 3];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1, 2, 3]
        );

        let source_vec = vec![1, 1, 1];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1]
        );
    }
}
