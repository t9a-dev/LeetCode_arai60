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
  - Pythonには商と余りを求めるdivmodメソッドがあるそう。Rustにはないのでタプルで返すと同じ感じにできそうだと思った。
  https://github.com/TrsmYsk/leetcode/pull/7/files#r2422727971

  - 次の実装がシンプルで分かりやすいので参考にすることにした。
  https://github.com/docto-rin/leetcode/pull/5/files#diff-d1bf28eecba1030ec5c99bef0bcf26b76d53d4d967eb4d2768ee5e173111fb19R130-R150

  - 他の方の実装例がまとまっており見やすい
  https://github.com/docto-rin/leetcode/pull/5/files#diff-d1bf28eecba1030ec5c99bef0bcf26b76d53d4d967eb4d2768ee5e173111fb19R56-R83

  改善する時に考えたこと
  - 前回取り組んだ 82.Remove Duplicates from Sorted List II　でtailの部分が理解できなかったので番兵で実装してみる。
  https://github.com/t9a-dev/LeetCode_arai60/pull/10
  https://github.com/t9a-dev/LeetCode_arai60/pull/10#discussion_r2449633514
  https://github.com/t9a-dev/LeetCode_arai60/pull/10#discussion_r2449649287
  nextが常にNoneとなるようなノードをtailとすることで、末尾を表しているという理解で進めてみる。
  初期化時tailはheadと同じアドレスを参照しているが、tailだけ次のノードに参照先を移動していく。

  所感
  - 別の問題でもらったレビューにより番兵が理解できた。
  https://github.com/t9a-dev/LeetCode_arai60/pull/10#discussion_r2449633514

  - 上記の実装を理解したと感じてからスムーズに実装できたので番兵による実装が理解できたと思った。
    - tailが理解できなかったのは、ストリーム方式で捉えていたからだと思った。
      - ストリーム方式では入力自体をin-placeで今見ているノードを指し示しながら走査していくが、このとき指し示しているノードが末尾とは限らないのでtailでは無いだろうと感じていたのだと思う。
      - 番兵方式ではストリーム方式と違い、ダミー（番兵）のを生成してこれに対して後ろにノードを継ぎ足していく方式であると理解した。つまりどこに継ぎ足すかをtailが常に指し示している。
        - ダミーの末尾（次にノードがない）を表すためにtailという表現がされていると理解した。
  - 空間計算量O(1)にするには入力どちらかをin-placeで扱う必要があるが、入力に変更を加えるのは適切でないと思った。
  add_two_numbersメソッドの引数がimmutableであること、メソッド命から入力が破壊されるとは考えられないため。
*/

pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1_cursor, mut l2_cursor) = (l1, l2);
        let mut sentinel_head = Box::new(ListNode::new(0));
        let mut sentinel_tail = sentinel_head.as_mut();
        let mut carry = 0;

        while carry != 0 || l1_cursor.is_some() || l2_cursor.is_some() {
            let (l1_node, l2_node) = (
                l1_cursor.unwrap_or(Box::new(ListNode::new(0))),
                l2_cursor.unwrap_or(Box::new(ListNode::new(0))),
            );
            let sum = l1_node.val + l2_node.val + carry;
            let (current_carry, node_val) = (sum / 10, sum % 10);

            carry = current_carry;
            sentinel_tail = sentinel_tail.next.insert(Box::new(ListNode::new(node_val)));
            l1_cursor = l1_node.next;
            l2_cursor = l2_node.next;
        }

        sentinel_head.next
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
        let (l1_values, l2_values) = (vec![2, 4, 3], vec![5, 6, 4]);
        let (l1, l2) = (vec_to_list_node(&l1_values), vec_to_list_node(&l2_values));
        assert_eq!(list_node_to_vec(&l1), l1_values);
        assert_eq!(list_node_to_vec(&l2), l2_values);
        assert_eq!(
            list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![7, 0, 8]
        );

        let (l1_values, l2_values) = (vec![0], vec![0]);
        let (l1, l2) = (vec_to_list_node(&l1_values), vec_to_list_node(&l2_values));
        assert_eq!(list_node_to_vec(&l1), l1_values);
        assert_eq!(list_node_to_vec(&l2), l2_values);
        assert_eq!(
            list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![0]
        );

        let (l1_values, l2_values) = (vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]);
        let (l1, l2) = (vec_to_list_node(&l1_values), vec_to_list_node(&l2_values));
        assert_eq!(list_node_to_vec(&l1), l1_values);
        assert_eq!(list_node_to_vec(&l2), l2_values);
        assert_eq!(
            list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
