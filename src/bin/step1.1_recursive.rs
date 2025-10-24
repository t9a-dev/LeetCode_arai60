// Step1.1
// 目的: コードの変形練習のため別の解法で実装する

/*
  何を考えて解いていたか
  - この解法では番兵(dummy)とスタック、再帰を利用している。
  - step1のstackを用いた実装は特に改善点もないと思ったので練習のために再帰を使った実装を行ってみる。
    - nodeを引数で受け取ってnextがNoneになったら終了する条件で再帰を書けそう。
    - stackをpopしていって空になったら終了する条件で再帰を書けそう。
  - build_list_nodeでライフタイム識別子を使う必要が出てきてコンパイルを通すのに時間がかかった。

  正解してから気づいたこと
  - step1のループ部分を再帰処理にしただけになっている。変形しきれていない。
  - 再帰処理で少し違う実装方法を思いついたのでそれも試す。(step1.2_recursive.rs)
  具体的には、headが与えられたときにnextがNoneになるまで再帰で潜っていって、ListNodeを返しながら戻るイメージ。
*/

/*
  Nは入力全体のサイズとする。
  時間計算量: O(N) 入力のノードを全走査している。
  空間計算量: O(N) 入力のノードの値を全てスタックに積んでいる。
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
        let mut node_values = Vec::new();
        let mut dummy = Box::new(ListNode::new(0));
        let tail = dummy.as_mut();

        node_values = Self::collect_node_values(head, &mut node_values).to_vec();
        Self::build_reverse_list(tail, &mut node_values);

        dummy.next
    }

    fn collect_node_values(
        node: Option<Box<ListNode>>,
        node_values: &mut Vec<i32>,
    ) -> &mut Vec<i32> {
        if let Some(node) = node {
            node_values.push(node.val);
            return Self::collect_node_values(node.next, node_values);
        }

        node_values
    }

    fn build_reverse_list<'a>(
        mut tail: &'a mut ListNode,
        node_values: &'a mut Vec<i32>,
    ) -> &'a mut ListNode {
        if let Some(v) = node_values.pop() {
            let node = Box::new(ListNode::new(v));
            tail = tail.next.insert(node);
            return Self::build_reverse_list(tail, node_values);
        }

        tail
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
    fn step1_recursive_test() {
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
