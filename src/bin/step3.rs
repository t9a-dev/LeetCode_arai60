// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N=入力全体のサイズ
  時間計算量: O(N) 入力のノードを全走査するので
  空間計算量: O(1) インプレイスで入力をそのまま変更して返すので
*/

/*
 1回目: 3分14秒
 2回目: 4分17秒
 3回目: 2分33秒
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {
            while let Some(next_node) = node.next.as_mut() {
                if node.val != next_node.val {
                    break;
                }
                node.next = next_node.next.take();
            }
            current = node.next.as_mut();
        }

        head
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
    fn step3_test() {
        let source_vec = vec![1, 1, 2];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1, 2]
        );

        let source_vec = vec![1, 1, 2, 3, 3];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1, 2, 3]
        );

        let source_vec = vec![1, 1, 1];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1]
        );
    }
}
