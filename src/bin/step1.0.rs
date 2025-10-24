// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - ノードが持つ値で降順ソートされた単方向リンクリストが与えられる。
  与えられた単方向リンクリストの値に基づいて昇順ソートした単方向リンクリストを返す。

  何を考えて解いていたか
  - 走査しながらstackにプッシュしていく。(out-of-place処理)
  - stackからpopしながらdummy(番兵)ノードにつなげていく。
  - dummy.nextを返す。
  - 問題が簡単に感じられるので、in-placeによる実装と再帰処理による実装もしてみる。

  正解してから気づいたこと
  - 特に迷うこと無く初めて一発で通った。
  - この解法では番兵(dummy)とスタックを利用している。
  - 改善する点も見当たらないので再帰処理を利用した実装を練習する。
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
        let mut target_node = head;
        let mut node_values = vec![];
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = dummy.as_mut();

        while let Some(node) = target_node {
            node_values.push(node.val);
            target_node = node.next;
        }

        while let Some(node_value) = node_values.pop() {
            tail = tail.next.insert(Box::new(ListNode::new(node_value)));
        }

        dummy.next
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
    fn step1_sentinel_and_stack_test() {
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
