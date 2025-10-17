// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  入力のリンクリストのサイズをNとする
  時間計算量: O(N) 全走査するため
  空間計算量: O(N) 全走査しながらHashSetに記録していくため
*/

/*
 1回目: 1分51秒

 1回目で問題の解を書くだけだと2分かからなかったので、
  - 単方向リンクリストの構造体(pub struct ListNode)
  - テストコードで利用する単方向リンクリスト生成処理(fn build_list_with_cycle)
  - 問題の解(fn has_cycle)
 以降は上記を書く時間を計測する

 2回目: 11分2秒
 3回目: 8分52秒
 4回目: 8分40秒
 5回目: 8分6秒
*/

use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct ListNode {
    next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution {}
impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut visited_nodes = HashSet::new();
        let mut next_node = head;

        while let Some(current_node) = next_node {
            if !visited_nodes.insert(current_node.as_ptr()) {
                return true;
            }

            next_node = current_node.borrow().next.as_ref().map(Rc::clone);
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
        if 0 == list_len {
            panic!("invalid list_len require 1");
        }

        let nodes = (0..list_len)
            .map(|_| Rc::new(RefCell::new(ListNode { next: None })))
            .collect::<Vec<_>>();
        let tail_position = list_len - 1;

        for (i, node) in nodes.iter().enumerate() {
            if let Some(next_node) = nodes.get(i + 1) {
                node.borrow_mut().next = Some(Rc::clone(&next_node));
            }
        }

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
    fn step3_no_cycle_test() {
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
    fn step3_with_cycle_test() {
        let expect = true;
        let with_cycle = build_list_with_cycle(Some(1), 4);
        assert_eq!(Solution::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(1), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);

        let with_cycle = build_list_with_cycle(Some(2), 3);
        assert_eq!(Solution::has_cycle(with_cycle), expect);
    }
}
