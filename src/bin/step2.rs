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
  - 様々な解法が検討されており、非常に参考になる。
  https://github.com/yas-2023/leetcode_arai60/pull/4/files

  - LeetCodeでVimモードがあるの知らなかったので助かった。
  https://github.com/yas-2023/leetcode_arai60/pull/4/files#diff-a2c7d2692707bbe67e316042ed9e36c45f92db461cb104915bedc6638dd9aa26R41-R43

  - インプレイスでの実装を理解するのに以下の実装を参考にすることにしたが、結局理解ができなかった。
  dummy,tailのうちtailがよくわからないのが原因だと考えられる。（コードが悪いということではない）
  https://github.com/docto-rin/leetcode/pull/4/files#diff-2bdfe1140252df4bf36c06f29251eeade51991cec8bee544c92c5b27c63cfc7aR147-R197
  インプレイスで処理する方法もstep2_1_in-place.rsで実装してみる

  改善する時に考えたこと
  - step1で二重の全走査部分をHashMapを用いた方法に書き換えて処理全体の時間計算量をO(N^2) -> O(N)に改善した。
    - ナイーブな実装だが何をしているのかは分かりやすいと思う。
*/

/*
  Nは入力のサイズとする
  時間計算量: O(N)
  空間計算量: O(N)
*/

use std::collections::HashMap;

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
