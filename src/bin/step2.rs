// Step2
// 目的: コードの変形練習のため別の解法で実装する

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
  - in-placeの処理は思いつかなかったので実装してみる。
  読み進める時に解法自体にお手玉感を感じたが、in-placeで空間計算量をO(1)に節約できるのとトレードオフできるくらいの複雑さだなと思った。
  https://github.com/nanae772/leetcode-arai60/pull/8/files#diff-b8c59da17cc50c9a7166ed6e8fc6e9771cb3d45105ede43900b25a3b1acfa22fR9-R18

  所感など
  - この解法を見たときすぐに理解できず、図で値の動き方をトレースして初めて理解できた。
  - 番兵も使っている(reversed_head)
  - reversed_headに値を詰めていっているように見えたので空間計算量がO(N)になっているように見えたがそうではない。
  参照を移動しているだけであり、データの実体があるヒープ領域は触っていない。
  Box<T>はスマートポインタ※を返すので、ポインタ自体を取り回しても空間計算量は消費しない。※ヒープ領域に置いてある実データのアドレスや所有権の情報を持っている
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
        let mut cursor = head;
        let mut reversed_head = None;

        while let Some(mut node) = cursor {
            let next_node = node.next.take();
            node.next = reversed_head;
            reversed_head = Some(node);
            cursor = next_node;
        }

        reversed_head
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
    fn step2_test() {
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
