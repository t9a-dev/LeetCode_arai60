// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  Nは
    - 循環していない場合は開始ノード(head)からnextを持たないノードまでのサイズ
    - 循環している場合は開始ノード(head)から循環開始ノードまでのサイズ
  とする。
  時間計算量: O(N)
  空間計算量: O(N)
*/

/*
  1回目: 5分28秒
  2回目: 4分38秒
  3回目: 3分34秒
*/

/*
  フロイドの循環検出法は複雑だと思うので採用しない。
*/

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct ListNode {
    next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution {}
impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<usize> {
        let mut node_position_by_pointer: HashMap<_, _> = HashMap::new();
        let mut node_position_count = 0;
        let mut next_node = head;

        while let Some(current_node) = next_node {
            if let Some(cycle_node_position) = node_position_by_pointer.get(&current_node.as_ptr())
            {
                return Some(*cycle_node_position);
            }

            node_position_by_pointer
                .entry(current_node.as_ptr())
                .or_insert(node_position_count);
            next_node = current_node.borrow().next.as_ref().map(Rc::clone);
            node_position_count += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list_with_cycle(
        cycle_position: Option<usize>,
        list_len: usize,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if list_len == 0 {
            panic!("invalid list_len require 1");
        }
        let nodes = (0..list_len)
            .map(|_| Rc::new(RefCell::new(ListNode { next: None })))
            .collect::<Vec<_>>();
        let tail_position = list_len - 1;

        for (i, node) in nodes.iter().enumerate() {
            if let Some(next_node) = nodes.get(i + 1) {
                node.borrow_mut().next = Some(Rc::clone(next_node));
            }
        }

        if let Some(cycle_position) = cycle_position {
            if let (Some(tail_node), Some(cycle_to_node)) =
                (nodes.get(tail_position), nodes.get(cycle_position))
            {
                tail_node.borrow_mut().next = Some(Rc::clone(cycle_to_node));
            }
        }

        Some(Rc::clone(&nodes[0]))
    }

    #[test]
    fn step3_no_cycle_test() {
        let expect = None;

        let head = build_list_with_cycle(None, 1);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(3), 2);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(2), 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_cycle_test() {
        let expect_cycle_position = Some(7);
        let head = build_list_with_cycle(expect_cycle_position, 8);
        assert_eq!(Solution::detect_cycle(head), expect_cycle_position);

        let expect_cycle_position = Some(0);
        let head = build_list_with_cycle(expect_cycle_position, 2);
        assert_eq!(Solution::detect_cycle(head), expect_cycle_position);

        let expect_cycle_position = Some(0);
        let head = build_list_with_cycle(expect_cycle_position, 1);
        assert_eq!(Solution::detect_cycle(head), expect_cycle_position);
    }

    // 以下、ChatGPT(GPT-5)で生成
    #[test]
    #[should_panic(expected = "invalid list_len require 1")]
    fn step3_should_panic_on_zero_length() {
        let _ = build_list_with_cycle(None, 0);
    }

    #[test]
    fn step3_cycle_at_tail_self_loop() {
        let list_len = 5;
        let expect = Some(list_len - 1);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_cycle_in_middle() {
        let list_len = 10;
        let expect = Some(4);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_two_nodes_cycle_to_tail() {
        let expect = Some(1);
        let head = build_list_with_cycle(expect, 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_long_list_no_cycle() {
        let head = build_list_with_cycle(None, 1000);
        assert_eq!(Solution::detect_cycle(head), None);
    }

    #[test]
    fn step3_long_list_cycle_early() {
        let list_len = 200;
        let expect = Some(3);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_long_list_cycle_late() {
        let list_len = 200;
        let expect = Some(198);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step3_out_of_range_cycle_position_means_no_cycle() {
        let head = build_list_with_cycle(Some(9999), 5);
        assert_eq!(Solution::detect_cycle(head), None);

        let head = build_list_with_cycle(Some(10), 10);
        assert_eq!(Solution::detect_cycle(head), None);
    }
}
