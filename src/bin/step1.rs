// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 符号付き整数を値として持つソート済の単方向リンクリストが与えられる。
  単方向リンクリストの値に着目して一意の値を持つノードのみを抽出して返す。

  何がわからなかったか
  - 入力をインプレイスで変更して返す方法。できそうな気はするが5分では具体的な方法まで思いつかなかった。

  何を考えて解いていたか
  - リンクリストを全走査しながら、ノードの値を配列に集める。
  配列からcontainsのカウントが1になる値のみを抽出してソート後、リンクリストを生成して返す。

  正解してから気づいたこと
  - Rustのイテレータメソッドのcontainsではカウントを返さないことを知った。
  - 問題を見たときに思いついた解法では時間計算量がO(N^2)になっているが、O(N)にはできそうな感じはする。
    - HashMapを利用してキーにノードの値、値に出現回数を入れていき、出現回数が1の値で返却用のリンクリストを生成する方法でできる。
*/

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
        let mut node_values: Vec<_> = Vec::new();
        let mut target_node = head;

        while let Some(node) = target_node {
            node_values.push(node.val);
            target_node = node.next;
        }

        let single_occurrence_values = node_values
            .iter()
            .filter_map(|v| {
                if node_values.iter().filter(|x| v == *x).count() == 1 {
                    return Some(v);
                }
                None
            })
            .collect::<Vec<_>>();

        single_occurrence_values
            .iter()
            .rev()
            .fold(None, |child, v| {
                let mut parent = Box::new(ListNode::new(**v));
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
    fn step1_test() {
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
