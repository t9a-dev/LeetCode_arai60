// step2_1_1
// 目的: フロイドの循環検出法による実装をしておく

/*
  フロイドの循環検出法:
    slow,fastは同じ位置からスタートする。slowは1サイクルごとに1ステップ、fastは1サイクルごとに2ステップ進む。
    衝突を検知したら循環ノードのポジション算出処理に移行する。
    循環ノードの算出処理では、衝突検知地点とノード先頭から1サイクル1ステップずつ進んでいく。
    衝突するまでのサイクル数がノードのポジションとなる。
*/

/*
  Nは
    - 循環していない場合は開始ノード(head)からnextを持たないノードまでのサイズ
    - 循環している場合は開始ノード(head)から循環開始ノードまでのサイズ
  とする。
  時間計算量: O(N)
  空間計算量: O(1)　リンクリストを辿りながらインプレイスで判定するので（ハッシュテーブルなどを生成しない）
*/

/*
  所感
  - なぜこうなるのかの数学的な証明はできないが、こうするとこうなるといった感じで実装した。
  - だいぶ複雑に感じるので、この部分がボトルネックになるようなシステムやハードウェアに近い低レイヤーでのプログラミングでしか採用しないかなと思った。
  - 循環検知後のノードのポジションを探す処理は関数に切り出そうかと思ったが、この処理はノードのポジションの初期条件が整っていないと無限ループになるので、
  関数に切り出して再利用可能な形にするのは止めた。
*/

use std::{cell::RefCell, rc::Rc};

pub struct ListNode {
    next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution {}
impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<usize> {
        let mut slow = Self::next_node(head.as_ref().map(Rc::clone));
        let mut fast = Self::next_node(Self::next_node(head.as_ref().map(Rc::clone)));

        while let (Some(slow_node), Some(fast_node)) = (slow, fast) {
            if Rc::ptr_eq(&slow_node, &fast_node) {
                // find cycle node position
                let mut node_position_count = 0;
                let mut current = head.as_ref().map(Rc::clone);
                let mut cycled = Some(Rc::clone(&fast_node));

                while let (Some(current_node), Some(cycled_node)) = (current, cycled) {
                    if Rc::ptr_eq(&current_node, &cycled_node) {
                        return Some(node_position_count);
                    }

                    current = Self::next_node(Some(current_node));
                    cycled = Self::next_node(Some(cycled_node));
                    node_position_count += 1;
                }
            }

            slow = Self::next_node(Some(slow_node));
            fast = Self::next_node(Self::next_node(Some(fast_node)));
        }

        None
    }

    fn next_node(node: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        node.and_then(|n| n.borrow().next.as_ref().map(Rc::clone))
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
    fn step2_1_no_cycle_test() {
        let expect = None;

        let head = build_list_with_cycle(None, 1);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(3), 2);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(2), 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_cycle_test() {
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
    fn step2_1_should_panic_on_zero_length() {
        let _ = build_list_with_cycle(None, 0);
    }

    #[test]
    fn step2_1_cycle_at_tail_self_loop() {
        let list_len = 5;
        let expect = Some(list_len - 1);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_cycle_in_middle() {
        let list_len = 10;
        let expect = Some(4);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_two_nodes_cycle_to_tail() {
        let expect = Some(1);
        let head = build_list_with_cycle(expect, 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_long_list_no_cycle() {
        let head = build_list_with_cycle(None, 1000);
        assert_eq!(Solution::detect_cycle(head), None);
    }

    #[test]
    fn step2_1_long_list_cycle_early() {
        let list_len = 200;
        let expect = Some(3);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_long_list_cycle_late() {
        let list_len = 200;
        let expect = Some(198);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step2_1_out_of_range_cycle_position_means_no_cycle() {
        let head = build_list_with_cycle(Some(9999), 5);
        assert_eq!(Solution::detect_cycle(head), None);

        let head = build_list_with_cycle(Some(10), 10);
        assert_eq!(Solution::detect_cycle(head), None);
    }
}
