// Step1
// 目的: コードの変形練習のため別の解法で実装する

/*
  何を考えて解いていたか
  - headが与えられたときにnextがNoneになるまで再帰で潜っていって、ListNodeを返しながら戻る方法を思いついてので実装してみる。
  この方向で実装できそうな気はしたが、自分で実装できなかったのでChatGPTに実装してもらって写経した。
  コードは読めるが実装で詰まるので、再帰処理の実装に慣れていく段階なのかなと思った。

  正解してから気づいたこと
  - step1_recursive.rsと比べたときに番兵を使わず、再帰処理だけで実装できているのでこちらの方がシンプルで分かりやすいと思った。
*/

/*
  Nは入力全体のサイズとする。
  時間計算量: O(N) 入力のノードを全走査している。
  空間計算量: O(1) 入力の参照移動だけで済んでおり追加のメモリ領域を確保していない。
*/

pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_list_node(None, head)
    }

    fn reverse_list_node(
        previous: Option<Box<ListNode>>,
        current: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match current {
            Some(mut node) => {
                let next_node = node.next.take();
                node.next = previous;
                Self::reverse_list_node(Some(node), next_node)
            }
            None => previous,
        }
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
        let mut current_node = head;

        while let Some(node) = current_node {
            out.push(node.val);
            current_node = &node.next;
        }

        out
    }

    #[test]
    fn step1_2_recursive_test() {
        let source_vec = vec![5, 4, 3, 2, 1];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::reverse_list(head)),
            vec![1, 2, 3, 4, 5]
        );

        let source_vec = vec![1, 2, 3, 4, 5];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::reverse_list(head)),
            vec![5, 4, 3, 2, 1]
        );

        let source_vec = vec![];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(list_node_to_vec(&Solution::reverse_list(head)), vec![]);
    }
}
