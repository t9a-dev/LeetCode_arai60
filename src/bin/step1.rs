// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  142. Linked-List-Cycle-Ⅱ
  問題の理解
  - 単方向リンクリストが与えられる。
  循環している場合、最後尾ノードがリンクしている循環先ノードのインデックスを返す。
  循環していない場合nullを返す。(RustなのでNone)

  何がわからなかったか
  - フロイドの循環検出法ではカウントのインクリメント周りがうまくできなかった。
  循環先ノードのポジションを特定することを要求されたことにより、フロイドの循環検出法をきちんと理解できていないことが露呈した。

  何を考えて解いていたか
  - この問題もRustだとLeetCodeで検証できないので、まずはテストから書く。
  141. Linked List Cycleで使ったテストコードを流用できる。
  単方向リンクリストの生成処理で循環先ノードのインデックスを指定するので、戻り値のインデックスが生成時のインデックスと等しいかで検証する。

  - ListNodeを全走査する。ListNodeのnextでNoneを見つけた時点で循環していないのでNoneを返す。
  HashMapのキーにポインタ、値に走査ループ回数を記録しておきハッシュマップからポインターで引いて見つかれば循環しているのでpositionとして返す。

  正解してから気づいたこと
  - フロイドの循環検出法では解けないものの、HashMapを利用した方法では自力で解けたのは良かった。
  - HashMap<_, usize>はHashMap<_, _>の方がコンパイラに型推論させる方がすっきりして良い。
  - ループの中でハッシュマップに対してget操作が重複(entryもキーが存在するか見てる)しているので、.and_then().or_else()やmatch文でまとめられないかと思ったが、
  以下の理由からこのままで良いと思った。
    - 既存のコードはループ先頭でガード節の役割を果たしている
    - ハッシュマップのキーはポインター(usize)で固定長なので操作の時間計算量は可読性を犠牲にするほどではない
*/

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct ListNode {
    next: Option<Rc<RefCell<ListNode>>>,
}

pub struct Solution {}
impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<usize> {
        let mut node_position_by_pointer: HashMap<_, usize> = HashMap::new();
        let mut node_position_count = 0;
        let mut next_node = head;

        while let Some(current_node) = next_node {
            if let Some(cycle_node_position) =
                node_position_by_pointer.get(&Rc::clone(&current_node).as_ptr())
            {
                return Some(*cycle_node_position);
            }

            node_position_by_pointer
                .entry(Rc::clone(&current_node).as_ptr())
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
    fn step1_no_cycle_test() {
        let expect = None;

        let head = build_list_with_cycle(None, 1);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(3), 2);
        assert_eq!(Solution::detect_cycle(head), expect);

        let head = build_list_with_cycle(Some(2), 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_cycle_test() {
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
    fn step1_should_panic_on_zero_length() {
        let _ = build_list_with_cycle(None, 0);
    }

    #[test]
    fn step1_cycle_at_tail_self_loop() {
        let list_len = 5;
        let expect = Some(list_len - 1);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_cycle_in_middle() {
        let list_len = 10;
        let expect = Some(4);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_two_nodes_cycle_to_tail() {
        let expect = Some(1);
        let head = build_list_with_cycle(expect, 2);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_long_list_no_cycle() {
        let head = build_list_with_cycle(None, 1000);
        assert_eq!(Solution::detect_cycle(head), None);
    }

    #[test]
    fn step1_long_list_cycle_early() {
        let list_len = 200;
        let expect = Some(3);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_long_list_cycle_late() {
        let list_len = 200;
        let expect = Some(198);
        let head = build_list_with_cycle(expect, list_len);
        assert_eq!(Solution::detect_cycle(head), expect);
    }

    #[test]
    fn step1_out_of_range_cycle_position_means_no_cycle() {
        let head = build_list_with_cycle(Some(9999), 5);
        assert_eq!(Solution::detect_cycle(head), None);

        let head = build_list_with_cycle(Some(10), 10);
        assert_eq!(Solution::detect_cycle(head), None);
    }
}
