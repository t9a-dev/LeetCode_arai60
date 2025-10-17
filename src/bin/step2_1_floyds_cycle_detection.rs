// Step2_1
// 目的: floyd's cycle detectionの実装をしてみる。

/*
  フロイドの循環検出法では２人の内1人が一歩先にいる状態から走査をスタートする。
  サイクルごとに2人の現在位置が同じでないかを確認していき同じであれば循環しているので走査を終了。
  サイクルごとにスタート位置が手前の方は一歩ずつ進む。一歩先にいる方は二歩進んでいく。
  一歩先の人が次のノードを見つけられなければ循環していないので終了。
  この方法では走査しながら確認できる（ワンパス）ので追加の補助空間計算量がかからず、空間計算量がO(1)となる。
  https://leetcode.com/problems/linked-list-cycle/solutions/4831939/brute-optimal-both-approach-full-explained-java-c-python3-rust-go-javascript/
*/

/*
  入力のリンクリストのサイズをNとする
  時間計算量: O(N) 全走査するため
  空間計算量: O(1) 全走査しながらインプレイスで重複チェックしており、HashSetなどに記録していないため
*/

use std::{cell::RefCell, rc::Rc};

pub struct ListNode {
    pub next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution {}
impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let (mut slow_node, mut fast_node) = {
            let current = head.as_ref().map(Rc::clone);
            let next = head.and_then(|head_node| head_node.borrow().next.as_ref().map(Rc::clone));

            (current, next)
        };

        while let (Some(slow), Some(fast)) = (slow_node, fast_node) {
            if Rc::ptr_eq(&slow, &fast) {
                return true;
            }

            slow_node = slow.borrow().next.as_ref().map(Rc::clone);
            fast_node = fast
                .as_ref()
                .borrow()
                .next
                .as_ref()
                .and_then(|next_node| next_node.as_ref().borrow().next.as_ref().map(Rc::clone));
        }

        false
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

        // nodeのnextに接続する
        for (i, node) in nodes.iter().enumerate() {
            if let Some(next_node) = nodes.get(i + 1) {
                node.borrow_mut().next = Some(Rc::clone(&next_node));
            }
        }

        //　tail_nodeのnextにcycle_positionで指定されたnodeを接続
        if let Some(cycle_position) = cycle_position {
            if let (Some(tail_node), Some(cycle_to_node)) =
                (nodes.get(tail_position), nodes.get(cycle_position))
            {
                tail_node.borrow_mut().next = Some(Rc::clone(&cycle_to_node));
            }
        }

        Some(Rc::clone(&nodes[0]))
    }

    #[test]
    fn step2_1_no_cycle_test() {
        let expect = false;
        let no_cycle = build_list_with_cycle(None, 4);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(None, 1);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(4), 2);
        assert_eq!(Solution::has_cycle(no_cycle), expect);

        let no_cycle = build_list_with_cycle(Some(2), 2);
        assert_eq!(Solution::has_cycle(no_cycle), expect);
    }

    #[test]
    fn step2_1_with_cycle_test() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(Solution::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);
    }
}
