// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 単方向リンクリストheadが与えられる。ノードはvalに符号付き整数を持っている。ノードはvalの値でソートされている。
  与えられた単方向リンクリストのノードのvalが一意になるように重複を取り除いた単方向リンクリストを出力する。

  何がわからなかったか
  - テストコードで利用しているヘルパー関数を自分で実装できずChatGPTを利用した。
  具体的には配列からBox<ListNode>を生成する過程でListNodeのnextに突っ込んでいくのがうまくできなかった。
  単方向リンクリストの末尾から生成していく発想がなく、テストコードにより実装の引き出しが増えた。

  何を考えて解いていたか
  - 与えられるheadの方がBox<ListNode>なので内部可変性を持たず、インプレイスで変更するのは無理そう。
  なので、返却用のListNodeを新たに生成する必要がある。
  - 単方向リンクリスト先頭から全走査していき、valを取り出してHashSetに入れていく。
  HashSetの値をvalに持つListNodeを生成して返す。

  正解してから気づいたこと
  - 入力がBox<T>でラップされており内部可変性を持たないので、インプレイスで全走査しながらnextを差し替えていくのは無理だと考えたが勘違いだった。
  少し調べたところ、内部可変性の概念を勘違いしていたことが分かった。
  共有参照を通して変更できないだけで、普通に可変参照を取得すればよいことが分かった。
  なので、渡された単方向リンクリストをインプレイスで走査して重複を削除できる。
  LinkedListCycle問題も上記の勘違いから、Rc<RefCell<T>>で内部可変性を持たせていたがBox<T>でできるはず。
*/

use std::collections::HashSet;

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
        let mut head = head;
        let mut unique_values: HashSet<i32> = HashSet::new();

        while let Some(node) = head {
            unique_values.insert(node.val);
            head = node.next;
        }

        let mut sorted_values = unique_values.into_iter().collect::<Vec<_>>();
        sorted_values.sort();

        sorted_values.into_iter().rev().fold(None, |child, v| {
            let mut parent = Box::new(ListNode::new(v));
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
            vec![1, 2]
        );

        let source_vec = vec![1, 1, 2, 3, 3];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![1, 2, 3]
        );
    }
}
