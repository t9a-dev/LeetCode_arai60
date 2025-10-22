// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  Nは入力の内大きい方のサイズ(max(l1,l2))
  時間計算量: O(N) 入力を全走査するため
  空間計算量: O(N) 入力のサイズ＋定数となり定数は無視される
*/

/*
  1回目: 7分00秒
  2回目: 5分24秒
  3回目: 5分22秒
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
    fn step3_test() {
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
