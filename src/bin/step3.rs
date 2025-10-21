// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  Nは入力全体のサイズとする
  時間計算量: O(N) 全走査するため
  空間計算量: O(N) HashMapで利用する補助空間計算量
*/

/*
  1回目: 6分49秒
  2回目: 5分22秒
  3回目: 4分8秒
*/

/*
  問題を見たときに自然に思いついた解法を採用した。
*/

use std::collections::HashMap;

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut frequency_by_node_value: HashMap<_, _> = HashMap::new();
        let mut target_node = head;

        while let Some(node) = target_node {
            frequency_by_node_value
                .entry(node.val)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);

            target_node = node.next;
        }

        let mut single_occurrence_values = frequency_by_node_value
            .keys()
            .filter_map(|node_value| match frequency_by_node_value.get(node_value) {
                Some(frequency) if *frequency == 1 => Some(node_value),
                _ => None,
            })
            .collect::<Vec<_>>();

        single_occurrence_values.sort();

        single_occurrence_values
            .into_iter()
            .rev()
            .fold(None, |child, node_value| {
                let mut parent = Box::new(ListNode::new(*node_value));
                parent.next = child;
                Some(parent)
            })
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
    fn step3_test() {
        let source_vec = vec![1, 1, 2];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![2]
        );

        let source_vec = vec![1, 1, 2, 3, 3];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![2]
        );

        let source_vec = vec![1, 1, 1];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(list_node_to_vec(&Solution::delete_duplicates(head)), vec![]);
    }
}
